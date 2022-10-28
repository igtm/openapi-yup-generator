use std::{fs, fs::File};
use std::io::Write;
use convert_case::{Case, Casing};
use jsonc_parser::{parse_to_value, JsonValue, JsonObject};
use std::collections::HashMap;
use clap::Parser;
use openapiv3::*;
use serde_yaml;
use serde_json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// openapi3 yaml file name 
   #[arg(short, long)]
   file: Option<String>,

   /// output file name
   #[arg(short, long)]
   out: Option<String>,

   /// config file name
   #[arg(short, long)]
   config: Option<String>,
}

static INITIAL_COMMENTS: &'static str = "/* tslint:disable */\n/* eslint-disable */\n//DO NOT EDIT MANUALLY\n\n";
static INITIAL_IMPORTS: &'static str = "import { object, string, number, date, array, bool } from 'yup';\n\n";
static FIELD_INDENTS: &'static str = "  ";
static FILE_NAME_INPUT_DEFAULT: &'static str = "openapi3.yaml";
static FILE_NAME_OUTPUT_DEFAULT: &'static str = "yup-defs.js";
static FILE_NAME_CONFIG: &'static str = "openapi-yup-generator-config.jsonc";

fn main() {
  // args
  let args = Args::parse();

  let mut config_file_name = FILE_NAME_CONFIG.to_owned();
  if let Some(i) =  args.config {
    config_file_name = i.to_string();
  }
  let config_string = fs::read_to_string(config_file_name).unwrap_or("{}".to_owned());
  let config_json_value = parse_to_value(&config_string, &Default::default()).unwrap().unwrap();
  let config = match config_json_value {
    JsonValue::Object(s) => s,
    _ => JsonObject::new(HashMap::new()),
  };

  let mut file = FILE_NAME_INPUT_DEFAULT.to_owned();
  if let Some(i) =  config.get_string("file") {
    file = i.to_string();
  }
  if let Some(i) =  args.file {
    file = i.to_string();
  }

  let mut out = FILE_NAME_OUTPUT_DEFAULT.to_owned();
  if let Some(o) =  config.get_string("out") {
    out = o.to_string();
  }
  if let Some(o) =  args.out {
    out = o.to_string();
  }
  let out_file = fs::File::create(out).unwrap();


  // openapiv3
  let f = std::fs::File::open(&file).unwrap();
  let openapi: OpenAPI;
  // detect ext
  if file.as_str().ends_with(".json") {
    openapi = serde_json::from_reader(f).expect("Could not deserialize input");
  } else if file.as_str().ends_with(".yaml") || file.as_str().ends_with(".yml") {
    openapi = serde_yaml::from_reader(f).expect("Could not deserialize input");
  } else {
    panic!("only json or yaml are supported, got {}.", file);
  }

  write_yup_defs(openapi, &config, out_file)

}

fn write_yup_defs(s: OpenAPI, config: &JsonObject, mut out_file: File) {
  let mut str = "".to_owned();

  // initial
  str.push_str(INITIAL_COMMENTS);
  str.push_str(INITIAL_IMPORTS);
  
  for a in &s.components {
    for (schema_name, or) in a.schemas.iter() {
      let scheme = or.as_item().unwrap();

      str += &format!("export const {} = object({{\n", schema_name.to_case(Case::UpperCamel));

      if let SchemaKind::Type(any_schema_type) = &scheme.schema_kind {
        if let Type::Object(any_schema_object_type) = any_schema_type {

          for (prop_name, p) in any_schema_object_type.properties.iter() {
            if let Some(any_schema_type2_item) = p.as_item() {
              if let SchemaKind::Type(any_schema_type2) = &any_schema_type2_item.schema_kind {

                match any_schema_type2 {
                  Type::Array(x) => {
                    str += &format!("{}{}: {}", FIELD_INDENTS, prop_name.to_case(Case::Camel), "array()");
                    // min/max
                    if let Some(minimum) = x.min_items {
                      str += &format!(".min({})", minimum);
                    }
                    if let Some(maximum) = x.max_items {
                      str += &format!(".max({})", maximum);
                    }
                  },
                  Type::Boolean{} => {
                    str += &format!("{}{}: {}", FIELD_INDENTS, prop_name.to_case(Case::Camel), "bool()");
                  },
                  Type::Integer(x) => {
                    str += &format!("{}{}: {}", FIELD_INDENTS, prop_name.to_case(Case::Camel), "number().integer()");
                    // min/max
                    if let Some(minimum) = x.minimum {
                      str += &format!(".min({})", minimum);
                    }
                    if let Some(maximum) = x.maximum {
                      str += &format!(".max({})", maximum);
                    }
                  },
                  Type::Number(x) => {
                    str += &format!("{}{}: {}", FIELD_INDENTS, prop_name.to_case(Case::Camel), "number()");
                    // min/max
                    if let Some(minimum) = x.minimum {
                      str += &format!(".min({})", minimum);
                    }
                    if let Some(maximum) = x.maximum {
                      str += &format!(".max({})", maximum);
                    }
                  },
                  Type::Object(x) => {
                    str += &format!("{}{}: {}", FIELD_INDENTS, prop_name.to_case(Case::Camel), "object()");
                  },
                  Type::String(x) => {
                    let mut type_name = "string".to_owned();
                    // type
                    if let VariantOrUnknownOrEmpty::Item(fmt_item) = &x.format {
                      match fmt_item {
                        StringFormat::Date | StringFormat::DateTime => {
                          type_name = "date".to_owned();
                        },
                        _ => {},
                      }
                    }
                    str += &format!("{}{}: {}()", FIELD_INDENTS, prop_name.to_case(Case::Camel), type_name);
                    // min/max
                    if let Some(minimum) = x.min_length {
                      str += &format!(".min({})", minimum);
                    }
                    if let Some(maximum) = x.max_length {
                      str += &format!(".max({})", maximum);
                    }
                    // matches (from pattern)
                    if let Some(pattern) = &x.pattern {
                      str += &format!(".matches(new RegExp(\"{}\"))", pattern);
                    }

                    // format
                    if let VariantOrUnknownOrEmpty::Unknown(fmt_name) = &x.format {
                      if fmt_name == "email" {
                        str += &format!(".{}()", fmt_name);
                      }
                    }
                  },
                }

                // [optional] label (from description)
                if let Some(_) =  config.get_boolean("description_as_label") {
                  if let Some(description) = &any_schema_type2_item.schema_data.description {
                    str += &format!(".label('{}')", description);
                  }
                }

                // required/optional
                if any_schema_object_type.required.iter().any(|pn| pn == prop_name) {
                  str += ".required()";
                } else {
                  str += ".optional()";
                }

                // end
                str += ",\n";
              }
            }
          }

        }
      }
      
      str += &format!("}});\n\n");
    }
  }

  out_file.write_all(str.as_bytes()).unwrap();

}
