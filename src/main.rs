mod cli;
mod generator;

use clap::Parser;
use cli::{Args,read_yaml};

fn main() {
    let args = Args::parse();

    let defs = read_yaml(&args.input);

    match args.target {
        cli::Target::Typescript => {
            generator::generate_typescript_code(&defs);
        }
        cli::Target::Python => {
            generator::generate_python_code(&defs);
        }
    }
}
