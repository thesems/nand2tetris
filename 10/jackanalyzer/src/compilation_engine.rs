use std::error::Error;

use crate::{
    tokenizer::{TokenType, Tokenizer},
    xml_writer::XmlWriter,
};

pub struct CompilationEngine<'a> {
    tokenizer: &'a mut Tokenizer,
    xml_writer: &'a mut XmlWriter,
}

impl<'a> CompilationEngine<'a> {
    pub fn build(
        tokenizer: &'a mut Tokenizer,
        xml_writer: &'a mut XmlWriter,
    ) -> Result<CompilationEngine<'a>, Box<dyn Error>> {
        tokenizer.reset();
        Ok(CompilationEngine {
            tokenizer,
            xml_writer,
        })
    }

    pub fn compile_class(&mut self) {
        self.xml_writer.write_full_tag("<class>");

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Keyword || self.tokenizer.token != "class" {
            panic!(
                "Error encountered. Expected keyword class, but received: {}",
                self.tokenizer.token
            );
        }
        self.write_token();

        self._compile_identifier("ERROR: expected a class identifier name after keyword 'class'.");

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "{" {
            panic!("Error encountered. Expected a symbol {{.",);
        }
        self.write_token();

        while !(self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "}") {
            self.tokenizer.advance();
            if self.tokenizer.token_type() != TokenType::Keyword {
                panic!("Received {}, but expected one of the following keywords: static, field, constructor, method, function.", self.tokenizer.token);
            }
            match self.tokenizer.token.as_str() {
            "static" | "field" => {
                self.compile_class_var_dec();
            }
            "constructor" | "function" | "method" => {
                self.compile_subroutine_dec();
            }
            _ => panic!("Received {}, but expected one of the following keywords: static, field, constructor, method, function.", self.tokenizer.token)
        };
        }

        self.xml_writer.write_full_tag("</class>");
    }

    pub fn compile_class_var_dec(&mut self) {
        self.xml_writer.write_full_tag("<classVarDec>");

        // static or field
        self.write_token();

        // keyword: variable type
        // TODO: class as a type
        self.tokenizer.advance();
        let allowed_keywords = vec!["int", "char", "boolean"];
        if self.tokenizer.token_type() != TokenType::Keyword
            || !allowed_keywords.contains(&self.tokenizer.token.as_str())
        {
            panic!(
                "Error encountered. Expected a variable type, but received: {}",
                self.tokenizer.token
            );
        }
        self.write_token();

        self._compile_identifier("ERROR: expected a variable identifier name after variable type.");

        loop {
            self.tokenizer.advance();
            if self.tokenizer.token_type() != TokenType::Symbol {
                panic!(
                    "ERROR: Expected an symbol, but received: {}!",
                    self.tokenizer.token
                );
            }

            if self.tokenizer.token == ";" {
                self.write_token();
                break;
            }

            if self.tokenizer.token != "," {
                panic!(
                    "ERROR: expected a symbol ',' but received: {}",
                    self.tokenizer.token
                );
            }

            self.write_token();

            // varName
            self._compile_identifier(
                "ERROR: expected a variable identifier name after ',' comma symbol.",
            );
        }

        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != ";" {
            panic!(
                "Error encountered. Expected an symbol, but received: {}!",
                self.tokenizer.token
            );
        }

        self.xml_writer.write_full_tag("</classVarDec>");
    }

    pub fn compile_subroutine_dec(&mut self) {
        self.xml_writer.write_full_tag("<subroutineDec>");

        let allowed_keywords = vec!["constructor", "function", "method"];
        if self.tokenizer.token_type() != TokenType::Keyword
            || !allowed_keywords.contains(&&self.tokenizer.token.as_str())
        {
            panic!("Error encountered. Expected a keyword: constructor, function or method, but received: {}", self.tokenizer.token);
        }
        self.write_token();

        // TODO: class as a type
        self.tokenizer.advance();
        let allowed_keywords = vec!["void", "int", "string", "bool"];
        if self.tokenizer.token_type() != TokenType::Keyword
            && !allowed_keywords.contains(&&self.tokenizer.token.as_str())
        {
            panic!(
                "Error encountered. Expected a variable type, but received: {}",
                self.tokenizer.token
            );
        }
        self.write_token();

        // subroutineName
        self._compile_identifier("ERROR: expected a subroutine identifier name.");

        self.compile_parameter_list();
        self.compile_subroutine_body();

        self.xml_writer.write_full_tag("</subroutineDec>");
    }

    pub fn compile_parameter_list(&mut self) {
        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "(" {
            panic!(
                "Expected an opening bracket '(' but received: {}",
                self.tokenizer.token
            );
        }
        self.write_token();

        // TODO: handle parameters

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != ")" {
            panic!(
                "Expected an closing bracket ')' but received: {}",
                self.tokenizer.token
            );
        }
        self.write_token();
    }

    pub fn compile_subroutine_body(&mut self) {
        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "{" {
            panic!(
                "ERROR: function body must start with an opening curly bracket '{{' but received: {}",
                self.tokenizer.token
            );
        }
        self.write_token();

        // TODO: handle parameters

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "}" {
            panic!(
                "ERROR: function body must start with an opening curly bracket '}}', but received: {}",
                self.tokenizer.token
            );
        }
        self.write_token();
    }

    pub fn compile_var_dec() {}

    pub fn compile_statements() {}

    pub fn compile_let() {}

    pub fn compile_if() {}

    pub fn compile_while() {}

    pub fn compile_do(&mut self) {
        self.write_token();
        // TODO: subroutineCall
    }

    pub fn compile_return(&mut self) {
        // Return
        self.write_token();

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol {
            self.compile_expression();
        }

        if self.tokenizer.token != ";" {
            panic!(
                "ERROR: expected a symbol ';' but received: {}.",
                self.tokenizer.token
            );
        }

        self.write_token();
    }

    pub fn compile_expression(&mut self) {}

    pub fn compile_term() {}

    pub fn compile_expression_list() {}

    fn _compile_identifier(&mut self, message: &str) {
        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Identifier {
            panic!("{}", message);
        }
        self.write_token();
    }

    fn write_token(&mut self) {
        self.xml_writer.write_token(
            self.tokenizer.token_type(),
            self.tokenizer.token.as_str(),
            self.tokenizer.int_token,
        );
    }
}
