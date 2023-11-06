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
        let input = args[1].clone();
        Config { input }
    };
    analyzer::run(config).unwrap_or_else(|err| {
        println!("Error occured: {}", err);
        process::exit(1);
    });
}
