use std::fs::File;
use std::io::{Result, Write};
use crate::tokenizer::TokenType;

pub struct XmlWriter {
    file: File
}
impl XmlWriter {
    pub fn build(out_path: &str) -> Result<XmlWriter> {
        let file = std::fs::File::options().create(true).write(true).open(out_path)?;
        Ok(XmlWriter { file })
    }

    pub fn write_token(&mut self, token_type: TokenType, token: &str, int_token: u16) {
        match token_type {
            TokenType::Keyword => {
                self.write_tags("keyword", token);
            }
            TokenType::Symbol => {
                let symbol = match token {
                    "<" => "&lt;",
                    ">" => "&gt;",
                    "\"" => "&quot;",
                    "&" => "&amp;",
                    _ => token
                };
                self.write_tags("symbol", symbol);
            }
            TokenType::IntConst => {
                self.write_tags("integerConstant", format!("{}", int_token).as_str());
            }
            TokenType::StringConst => {
                self.write_tags("stringConstant", token);
            }
            TokenType::Identifier => {
                self.write_tags("identifier", token);
            }
            _ => panic!("should not happen! received a token that is not handled.")
        }
    }

    pub fn write_full_tag(&mut self, tag_name: &str) {
        _ = write!(self.file, "{}\n", tag_name);
    }
    
    fn write_tags(&mut self, tag_name: &str, content: &str) {
        _ = write!(self.file, "<{}> {} </{}>\n", tag_name, content, tag_name);
    }
}
