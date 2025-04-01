use crate::cli::TypeDef;
use std::error::Error;

//TODO: Generate Python code
pub fn generate_python_code(defs: &[TypeDef]) -> Result<String, Box<dyn Error>>{
    //TODO: Generate Python code
    Err("Python code generation not implemented yet".into())
}


pub fn generate_typescript_code(defs: &[TypeDef]) -> String {
    let mut output = String::new();
    for def in defs {
        if def.export {
            output.push_str(&format!("export type {} = {{\n", capitalize(&def.name)));
        } else {
            output.push_str(&format!("type {} = {{\n", capitalize(&def.name)));
        }
        for (field, typ) in &def.types {
            output.push_str(&format!("  {}: {};\n", field, typ));
        }
        output.push_str("};\n\n");
    }
    output
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}