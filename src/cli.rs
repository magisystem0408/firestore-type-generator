use clap::{Parser, ValueEnum};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Parser, Debug)]
#[command(name="fitype",author, version)]
pub struct Args {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long)]
    pub output: String,

    #[arg(short, long)]
    pub target: Target,
}


#[derive(Debug, Clone, ValueEnum)]
pub enum Target {
    #[clap(name = "ts")]
    Typescript,

    #[clap(name = "py")]
    Python,
}

#[derive(Debug, Deserialize)]
pub struct TypeDef {
    pub name: String,
    pub export: bool,
    pub types: BTreeMap<String, String>,
}

pub fn read_yaml(path: &str) -> Vec<TypeDef> {
    let content = std::fs::read_to_string(path).expect("Failed to read YAML file");
    serde_yaml::from_str::<Vec<TypeDef>>(&content).expect("Failed to parse YAML content")
}