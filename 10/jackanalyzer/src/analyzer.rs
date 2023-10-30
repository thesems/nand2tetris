use std::fs;
use std::error::Error;

use crate::config::Config;
use crate::tokenizer::Tokenizer;
use crate::compilation_engine::CompilationEngine;

pub struct Analyzer {
    pub tokenizer: Tokenizer,
    pub comp_engine: CompilationEngine,
}

impl Analyzer {
    pub fn build(config: Config) -> Result<Analyzer, Box<dyn Error>> {
        let input = fs::read_to_string(config.input.as_str())?;
        let tokenizer = Tokenizer::build(input.as_str())?;
        let comp_engine = CompilationEngine::build(&tokenizer)?;

        Ok(Analyzer { tokenizer, comp_engine })
    }

    pub fn run(&self) {}
}

