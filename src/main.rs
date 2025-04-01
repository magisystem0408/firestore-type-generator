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
            if let Err(e) = std::fs::create_dir_all(&args.output) {
                eprintln!("Failed to create output directory: {}", e);
                std::process::exit(1);
            }
            if let Err(e) = std::fs::write(format!("{}/gen_types.ts", args.output), code) {
                eprintln!("Failed to write to output file: {}", e);
                std::process::exit(1);
            }
        }
        cli::Target::Python => {
            // TODO: after supporting Python code generation
            let _  = generator::generate_python_code(&defs);
        }
    }
}
