use std::fs;
use std::error::Error;

use crate::config::Config;
use crate::tokenizer::Tokenizer;
use crate::compilation_engine::CompilationEngine;
use crate::xml_writer::XmlWriter;

pub struct Analyzer {
}

impl Analyzer {
    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let ext = ".jack";
        let is_file = config.input.contains(ext);
        let mut input_files: Vec<String> = vec![];

        if !is_file {
            for entry in fs::read_dir(config.input)? {
                let entry = entry?;
                let file_name = String::from(entry.path().to_str().unwrap());

                if file_name.contains(ext) {
                    input_files.push(file_name.to_string());
                }
            }
        } else {
            input_files.push(config.input.clone());
        }

        while let Some(in_file) = input_files.pop() {
            println!("{}", in_file);
            if !in_file.contains(ext) {
                continue;
            }

            let out_xml_tok = in_file.replace(".jack", "T-gen.xml");

            let input = fs::read_to_string(in_file.as_str())?;
            let mut tokenizer = Tokenizer::build(input.as_str())?;
            let comp_engine = CompilationEngine::build(&tokenizer)?;
            let mut xml_writer = XmlWriter::build(out_xml_tok.as_str())?;

            xml_writer.write_full_tag("<tokens>")?;
            while tokenizer.has_more_tokens() {
                tokenizer.advance();
                xml_writer.write_token(tokenizer.token_type(), tokenizer.token.as_str(), tokenizer.int_token)?;
            }
            xml_writer.write_full_tag("</tokens>")?;
        }

        Ok(())
    }
}

