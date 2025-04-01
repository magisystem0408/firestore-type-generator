mod cli;
mod generator;

use clap::Parser;
use cli::{Args,read_yaml};

fn main() {
    let args = Args::parse();

    let defs = read_yaml(&args.input);

    match args.target {
        cli::Target::Typescript => {
            let code = generator::generate_typescript_code(&defs);
            std::fs::create_dir_all(&args.output).unwrap();
            std::fs::write(format!("{}/gen_types.ts", args.output), code).unwrap();
        }
        cli::Target::Python => {
            // TODO: after supporting Python code generation
            let _  = generator::generate_python_code(&defs);
        }
    }
}
