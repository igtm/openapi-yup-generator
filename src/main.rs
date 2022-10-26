use std::{fs, fs::File};
use std::io::Write;
use convert_case::{Case, Casing};
use jsonc_parser::{parse_to_value, JsonValue, JsonObject};
use std::collections::HashMap;
use clap::Parser;

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

  match oas3::from_path(file) {
    Ok(spec) => write_yup_defs(spec, &config, out_file),
    Err(err) => println!("error: {:?}", err)
  }
}

fn write_yup_defs(s: oas3::Spec, config: &JsonObject, mut out_file: File) {

  // initial
  out_file.write_all(INITIAL_COMMENTS.as_bytes()).unwrap();
  out_file.write_all(INITIAL_IMPORTS.as_bytes()).unwrap();
  
  for a in &s.components {
    for (schema_name, or) in a.schemas.iter() {
      let scheme = or.resolve( &s).unwrap();


      out_file.write_all(format!("export const {} = object({{\n", schema_name.to_case(Case::UpperCamel)).as_bytes()).unwrap();

      for (prop_name, p) in scheme.properties.iter() {
        let scheme2 = p.resolve( &s).unwrap();
        let schema_type_string = get_str(scheme2.schema_type);
        let format_type_string = get_str(scheme2.format);
        let mut str = format!("{}{}: {}()", FIELD_INDENTS, prop_name.to_case(Case::Camel), get_yup_type_name(&schema_type_string, &format_type_string));


        // format
        if let Some(fmt_name) = get_format_name(&schema_type_string, &format_type_string) {
          str += &format!(".{}()", fmt_name);
        }

        // min/max
        if let Some(minimum) = scheme2.minimum {
          str += &format!(".min({})", minimum);
        }
        if let Some(maximum) = scheme2.maximum {
          str += &format!(".max({})", maximum);
        }

        // matches (from pattern)
        if let Some(pattern) = scheme2.pattern {
          str += &format!(".matches(new RegExp(\"{}\"))", pattern);
        }

        // [optional] label (from description)
        if let Some(_) =  config.get_boolean("description_as_label") {
          if let Some(description) = scheme2.description {
            str += &format!(".label('{}')", description);
          }
        }
        
        // required/optional
        if scheme.required.iter().any(|pn| pn == prop_name) {
          str += ".required()";
        } else {
          str += ".optional()";
        }

        // end
        str += ",\n";
        out_file.write_all(str.as_bytes()).unwrap();
      }

      out_file.write_all(format!("}});\n\n").as_bytes()).unwrap();
    }
  }

}

// oas3 -> yup
fn get_yup_type_name<'a>(schema_type: &Option<String>, format_name: &Option<String>) -> String {
  return match schema_type {
    Some(schema_type_str) => match schema_type_str.as_str() {
      "Boolean" => "bool".to_owned(),
      "Integer" => "number".to_owned(),
      "Number" => "number".to_owned(),
      "String" => match format_name {
        Some(format_name_str) => match format_name_str.as_str() {
        "date" => "date".to_owned(),
        "date-time" => "date".to_owned(),
        _ => "string".to_owned(),
        },
        None => "string".to_owned(),
      },
      "Array" => "array".to_owned(),
      "Object" => "object".to_owned(),
      _ => "".to_owned(),
    }
    None => "".to_owned(),
  }
}

// oas3 -> yup
fn get_format_name<'a>(schema_type: &Option<String>, format_name: &Option<String>) -> Option<String> {
  return match schema_type {
    Some(schema_type_str) => match schema_type_str.as_str() {
      "Integer" =>  Some("integer".to_owned()),
      "String" => match format_name {
        Some(format_name_str) => match format_name_str.as_str() {
          "email" => Some("email".to_owned()),
          _ => None,
        },
        None => None,
      },
      _ => None,
    },
    None => None,
  }
}


fn get_str<'a, T : std::fmt::Debug>(s: Option<T>) -> Option<String> {
  if let Some(i) = s {
    return Some(format!("{:?}", i))
  }
  return None
}