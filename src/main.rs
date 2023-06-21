use clap::Parser;
use convert_case::{Case, Casing};
use jsonc_parser::{parse_to_value, JsonObject, JsonValue};
use once_cell::sync::Lazy;
use openapiv3::*;
use regex::Regex;
use std::collections::HashMap;
use std::io::Write;
use std::{fs, fs::File};

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

static INITIAL_COMMENTS: &str =
    "/* tslint:disable */\n/* eslint-disable */\n//DO NOT EDIT MANUALLY\n\n";
static INITIAL_IMPORTS: &str =
    "import { object, string, number, date, array, bool } from 'yup';\n\n";
static FIELD_INDENTS: &str = "  ";
static FILE_NAME_INPUT_DEFAULT: &str = "openapi3.yaml";
static FILE_NAME_OUTPUT_DEFAULT: &str = "yup-defs.js";
static FILE_NAME_CONFIG: &str = "openapi-yup-generator-config.jsonc";

fn main() {
    // args
    let args = Args::parse();

    let mut config_file_name = FILE_NAME_CONFIG.to_owned();
    if let Some(i) = args.config {
        config_file_name = i;
    }
    let config_string = fs::read_to_string(config_file_name).unwrap_or_else(|_| "{}".to_owned());
    let config_json_value = parse_to_value(&config_string, &Default::default())
        .unwrap()
        .unwrap();
    let config = match config_json_value {
        JsonValue::Object(s) => s,
        _ => JsonObject::new(HashMap::new()),
    };

    let mut file = FILE_NAME_INPUT_DEFAULT.to_owned();
    if let Some(i) = config.get_string("file") {
        file = i.to_string();
    }
    if let Some(i) = args.file {
        file = i;
    }

    let mut out = FILE_NAME_OUTPUT_DEFAULT.to_owned();
    if let Some(o) = config.get_string("out") {
        out = o.to_string();
    }
    if let Some(o) = args.out {
        out = o;
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
            let schema = resolve(&s, or).unwrap();

            str.push_str(&format!(
                "export const {} = ",
                schema_name.to_case(Case::UpperCamel)
            ));

            str.push_str(&get_str_from_schema(
                &s,
                config,
                schema,
                indent_str_curry("".to_owned()),
            ));

            str.push_str(";\n\n");
        }
    }

    out_file.write_all(str.as_bytes()).unwrap();
}

fn get_str_from_schema(
    s: &OpenAPI,
    config: &JsonObject,
    schema: &Schema,
    indent_str: impl Fn(String) -> String,
) -> String {
    let mut str = "".to_owned();

    if let SchemaKind::Type(any_schema_type2) = &schema.schema_kind {
        match any_schema_type2 {
            Type::Array(x) => {
                str.push_str("array()");
                // of
                if let Some(r) = &x.items {
                    if let Some(array_schema) = resolve(s, &r.to_owned().unbox()) {
                        str.push_str(".of(");
                        str.push_str(&get_str_from_schema(s, config, array_schema, indent_str));
                        str.push(')');
                    }
                }
                // min/max
                if let Some(minimum) = x.min_items {
                    str.push_str(&format!(".min({})", minimum));
                }
                if let Some(maximum) = x.max_items {
                    str.push_str(&format!(".max({})", maximum));
                }
            }
            Type::Boolean {} => {
                str.push_str("bool()");
            }
            Type::Integer(x) => {
                str.push_str("number().integer()");
                // min/max
                if let Some(minimum) = x.minimum {
                    str.push_str(&format!(".min({})", minimum));
                }
                if let Some(maximum) = x.maximum {
                    str.push_str(&format!(".max({})", maximum));
                }
            }
            Type::Number(x) => {
                str.push_str("number()");
                // min/max
                if let Some(minimum) = x.minimum {
                    str.push_str(&format!(".min({})", minimum));
                }
                if let Some(maximum) = x.maximum {
                    str.push_str(&format!(".max({})", maximum));
                }
            }
            Type::Object(x) => {
                str.push_str("object({\n");
                for (prop_name, p) in x.properties.iter() {
                    let nested_indent_str = &indent_str_curry(indent_str(FIELD_INDENTS.to_owned()));

                    // [optional] case
                    let mut cased_prop_name = prop_name.to_owned();
                    if let Some(i) = config.get_string("case") {
                        match i.to_string().as_str() {
                            "camel" => {
                                cased_prop_name = prop_name.to_case(Case::Camel);
                            }
                            "snake" => {
                                cased_prop_name = prop_name.to_case(Case::Snake);
                            }
                            _ => {}
                        }
                    }

                    // prop
                    str.push_str(&nested_indent_str(format!("{}: ", cased_prop_name)));

                    if let Some(any_schema_type2_item) = resolve(s, &p.to_owned().unbox()) {
                        str.push_str(&get_str_from_schema(
                            s,
                            config,
                            any_schema_type2_item,
                            nested_indent_str,
                        ));
                    }

                    // required/optional
                    if x.required.iter().any(|pn| pn == prop_name) {
                        str.push_str(".required()");
                    }

                    // end
                    str.push_str(",\n");
                }
                str.push_str(&indent_str("})".to_owned()));
            }
            Type::String(x) => {
                let mut type_name = "string".to_owned();
                // type
                if let VariantOrUnknownOrEmpty::Item(fmt_item) = &x.format {
                    match fmt_item {
                        StringFormat::Date | StringFormat::DateTime => {
                            type_name = "date".to_owned();
                        }
                        _ => {}
                    }
                }
                str.push_str(&format!("{}()", type_name));
                // min/max
                if let Some(minimum) = x.min_length {
                    str.push_str(&format!(".min({})", minimum));
                }
                if let Some(maximum) = x.max_length {
                    str.push_str(&format!(".max({})", maximum));
                }
                // matches (from pattern)
                if let Some(pattern) = &x.pattern {
                    str.push_str(&format!(".matches(new RegExp(\"{}\"))", pattern));
                }

                // format
                if let VariantOrUnknownOrEmpty::Unknown(fmt_name) = &x.format {
                    if fmt_name == "email" {
                        str.push_str(&format!(".{}()", fmt_name));
                    }
                }
            }
        }

        // [optional] label (from description)
        if config.get_boolean("description_as_label").is_some() {
            if let Some(description) = &schema.schema_data.description {
                str.push_str(&format!(".label('{}')", description));
            }
        }
    }

    str
}

static RE_REF: Lazy<Regex> = Lazy::new(|| {
    Regex::new("^(?P<source>[^#]*)#/components/(?P<type>[^/]+)/(?P<name>.+)$").unwrap()
});

fn resolve<'a>(s: &'a OpenAPI, ri: &'a ReferenceOr<Schema>) -> Option<&'a Schema> {
    match ri {
        ReferenceOr::Item(i) => Some(i),
        ReferenceOr::Reference { reference } => {
            let parts = RE_REF.captures(reference).unwrap();

            // find item which $ref points to
            for a in &s.components {
                for (schema_name, or) in a.schemas.iter() {
                    if &parts["name"] == schema_name {
                        return resolve(s, or);
                    }
                }
            }

            None
        }
    }
}

fn indent_str_curry(indent: String) -> impl Fn(String) -> String {
    move |str: String| format!("{}{}", indent, str)
}
