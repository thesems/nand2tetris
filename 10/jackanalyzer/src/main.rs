use std::{env, process};
pub mod config;
pub mod analyzer;
pub mod tokenizer;
pub mod compilation_engine;
pub mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::build(args[0].clone(), args[1].clone());
    let analyzer = analyzer::Analyzer::build(config).unwrap_or_else(|err| {
        process::exit(1);
    });
    
    analyzer.run();
}
