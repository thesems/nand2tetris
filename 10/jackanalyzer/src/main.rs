use std::{env, process};

use config::Config;
pub mod config;
pub mod analyzer;
pub mod tokenizer;
pub mod compilation_engine;
pub mod xml_writer;
pub mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = {
        let input = args[0].clone();
        let output = args[1].clone();
        let output_xml_tokenizer = args[2].clone();
        Config { input, output, output_xml_tokenizer }
    };
    let analyzer = analyzer::Analyzer::build(config).unwrap_or_else(|_err| {
        process::exit(1);
    });
    
    analyzer.run();
}
