use std::fs;
use std::error::Error;

use crate::config::Config;
use crate::tokenizer::Tokenizer;
use crate::compilation_engine::CompilationEngine;
use crate::xml_writer::XmlWriter;

pub struct Analyzer {
    pub tokenizer: Tokenizer,
    pub comp_engine: CompilationEngine,
}

impl Analyzer {
    pub fn build(config: Config) -> Result<Analyzer, Box<dyn Error>> {
        let input = fs::read_to_string(config.input.as_str())?;
        let mut tokenizer = Tokenizer::build(input.as_str())?;
        let comp_engine = CompilationEngine::build(&tokenizer)?;
        let mut xml_writer = XmlWriter::build(config.output_xml_tokenizer.as_str())?;

        xml_writer.write_full_tag("<tokens>")?;
        while tokenizer.has_more_tokens() {
            tokenizer.advance();
            xml_writer.write_token(tokenizer.token_type(), tokenizer.token.as_str(), tokenizer.int_token)?;
        }
        xml_writer.write_full_tag("</tokens>")?;

        Ok(Analyzer { tokenizer, comp_engine })
    }

    pub fn run(&self) {}
}

