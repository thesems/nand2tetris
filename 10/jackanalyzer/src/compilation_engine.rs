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
        self.xml_writer.write_full_tag("<parameterList>");
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
        self.xml_writer.write_full_tag("</parameterList>");
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

        loop {
            self.tokenizer.advance();
            if self.tokenizer.token_type() != TokenType::Keyword || self.tokenizer.token != "var" {
                break;
            }
            self.compile_var_dec();
        }

        self.xml_writer.write_full_tag("<statements>");
        self.compile_statements();
        self.xml_writer.write_full_tag("</statements>");

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "}" {
            panic!(
                "ERROR: function body must start with an closed curly bracket '}}', but received: {}",
                self.tokenizer.token
            );
        }
        self.write_token();
    }

    pub fn compile_var_dec(&mut self) {
        self.xml_writer.write_full_tag("<varDec>");
        self.write_token();

        self.tokenizer.advance();
        let allowed_keywords = vec!["int", "string", "bool"];

        if self.tokenizer.token_type() != TokenType::Identifier
            && self.tokenizer.token_type() != TokenType::Keyword
            && !allowed_keywords.contains(&&self.tokenizer.token.as_str())
        {
            panic!(
                "Error encountered. Expected a variable type, but received: {}",
                self.tokenizer.token
            );
        }
        self.write_token();

        self._compile_identifier(
            format!(
                "ERROR: expected a variable name identifier, but received {}.",
                self.tokenizer.token
            )
            .as_str(),
        );

        loop {
            self.tokenizer.advance();
            if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == ";" {
                self.write_token();
                break;
            }

            if !(self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == ",") {
                panic!(
                    "ERROR: expected the symbol ',' comma, but received: {}",
                    self.tokenizer.token
                );
            }

            self._compile_identifier(
                format!(
                    "ERROR: expected a variable name identifier, but received {}.",
                    self.tokenizer.token
                )
                .as_str(),
            );
        }
        self.xml_writer.write_full_tag("</varDec>");
    }

    pub fn compile_statements(&mut self) {
        loop {
            if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "}" {
                break;
            } else if self.tokenizer.token_type() != TokenType::Keyword {
                panic!("ERROR: a statement must include a keyword: let, if, while, do, return. Received {}.", self.tokenizer.token);
            }

            match self.tokenizer.token.as_str() {
                "let" => {
                    self.compile_let();
                }
                "if" => {
                    self.compile_if();
                }
                "while" => {
                    self.compile_while();
                }
                "do" => {
                    self.compile_do();
                }
                "return" => {
                    self.compile_return();
                }
                _ => {
                    panic!(
                        "ERROR: unrecongnized statement keyword: {}",
                        self.tokenizer.token
                    );
                }
            }
            self.tokenizer.advance();
        }
    }

    pub fn compile_let(&mut self) {
        self.xml_writer.write_full_tag("<letStatement>");
        self.write_token();

        self._compile_identifier("ERROR: expected an existing variable name.");

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol {
            panic!("ERROR: expected a symbol '=' or array deferencing '['.");
        }

        if self.tokenizer.token == "[" {
            self.write_token();

            self.tokenizer.advance();
            self.compile_expression();

            if self.tokenizer.token != "=" || self.tokenizer.token_type() != TokenType::Symbol {
                panic!("ERROR: expected an equal '=' symbol after variable name.");
            }
            self.write_token();
        } else if self.tokenizer.token == "=" {
            self.write_token();
        } else {
            panic!("ERROR: expected a symbol '=' or array deferencing '['.");
        }

        self.tokenizer.advance();
        self.compile_expression();

        if self.tokenizer.token != ";" || self.tokenizer.token_type() != TokenType::Symbol {
            panic!(
                "ERROR: expected a semi-colon ';' symbol, but received: {}",
                self.tokenizer.token
            );
        }
        self.write_token();

        self.xml_writer.write_full_tag("</letStatement>");
    }

    pub fn compile_if(&mut self) {
        self.xml_writer.write_full_tag("<ifStatement>");
        self.xml_writer.write_full_tag("</ifStatement>");
    }

    pub fn compile_while(&mut self) {
        self.xml_writer.write_full_tag("<whileStatement>");
        self.write_token();

        self.tokenizer.advance();
        if self.tokenizer.token != "(" || self.tokenizer.token_type() != TokenType::Symbol {
            panic!("ERROR: expected opening bracket after keyword while!");
        }
        self.write_token();

        self.tokenizer.advance();
        self.compile_expression();

        if self.tokenizer.token != ")" || self.tokenizer.token_type() != TokenType::Symbol {
            panic!("ERROR: expected closing bracket after keyword while!");
        }
        self.write_token();

        self.xml_writer.write_full_tag("</whileStatement>");
    }

    pub fn compile_do(&mut self) {
        self.xml_writer.write_full_tag("<doStatement>");
        self.write_token();
        // TODO: subroutineCall
        self.xml_writer.write_full_tag("</doStatement>");
    }

    pub fn compile_return(&mut self) {
        self.xml_writer.write_full_tag("<returnStatement>");

        self.write_token();

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol {
            self.compile_expression();

            if self.tokenizer.token_type() != TokenType::Symbol {
                panic!(
                    "ERROR: expected a semi-colon, but received {}.",
                    self.tokenizer.token
                );
            }
        }

        if self.tokenizer.token != ";" {
            panic!(
                "ERROR: expected a symbol ';' but received: {}.",
                self.tokenizer.token
            );
        }
        self.write_token();

        self.xml_writer.write_full_tag("</returnStatement>");
    }

    pub fn compile_expression(&mut self) {
        self.xml_writer.write_full_tag("<expression>");
        self.compile_term();

        let symbols = vec!["+", "-", "*", "/", "&", "|", "<", ">", "="];
        if self.tokenizer.token_type() == TokenType::Symbol
            && symbols.contains(&&self.tokenizer.token.as_str())
        {
            self.write_token();
            self.tokenizer.advance();

            self.compile_term();
        }

        self.xml_writer.write_full_tag("</expression>");
    }

    pub fn compile_term(&mut self) {
        self.xml_writer.write_full_tag("<term>");

        let keyword_constants = vec!["true", "false", "null", "this"];
        let unary_op = vec!["-", "~"];

        let safe_exit = |s: &mut CompilationEngine| {
            s.xml_writer.write_full_tag("</term>");
            s.tokenizer.advance();
        };

        if self.tokenizer.token_type() == TokenType::IntConst {
            self.write_token();
            safe_exit(self);
            return;
        } else if self.tokenizer.token_type() == TokenType::Keyword {
            if !keyword_constants.contains(&&self.tokenizer.token.as_str()) {
                panic!("ERROR: non-allowed keyword.");
            }
            self.write_token();
            safe_exit(self);
            return;
        } else if self.tokenizer.token_type() == TokenType::StringConst {
            self.write_token();
            safe_exit(self);
            return;
        } else if self.tokenizer.token_type() == TokenType::Symbol
            && unary_op.contains(&&self.tokenizer.token.as_str())
        {
            self.write_token();
            self.tokenizer.advance();
            self.compile_term();
            safe_exit(self);
            return;
        } else if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "(" {
            // TODO: handle ( expression )
            safe_exit(self);
        } else if self.tokenizer.token_type() == TokenType::Identifier {
            self.write_token();
        } else {
            panic!(
                "ERROR: expected an identifier but received {}",
                self.tokenizer.token
            );
        }

        self.tokenizer.advance();

        if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "[" {
            self.write_token();
            self.tokenizer.advance();
            self.compile_expression();

            self.tokenizer.advance();
            if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "]" {
                panic!("ERROR: expected a closing square bracket after expression.");
            }
            self.write_token();
        } else if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "(" {
            self.write_token();
            self.tokenizer.advance();
            self.compile_expression_list();

            self.tokenizer.advance();
            if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != ")" {
                panic!("ERROR: expected a closing bracket after expression list.");
            }
            self.write_token();
        } else if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "." {
            self.write_token();

            self._compile_identifier(
                "ERROR: expected a subroutine name identifier after dot '.' symbol.",
            );

            self.tokenizer.advance();
            if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "(" {
                panic!("ERROR: expected an opening bracket after expression list.");
            }
            self.write_token();

            self.compile_expression_list();

            self.tokenizer.advance();
            if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != ")" {
                panic!("ERROR: expected a closing bracket after expression list.");
            }
            self.write_token();
        } else {
            self.xml_writer.write_full_tag("</term>");
            return;
        }
       
        self.tokenizer.advance();
        self.xml_writer.write_full_tag("</term>");
    }

    pub fn compile_expression_list(&mut self) {
        self.xml_writer.write_full_tag("<expressionList>");
        self.xml_writer.write_full_tag("</expressionList>");
    }

    fn _compile_identifier(&mut self, message: &str) {
        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Identifier {
            panic!("{}", message);
        }
        self.write_token();
    }

    fn _compile_subroutine_call(&mut self) {
        self._compile_identifier("ERROR: expected a identifier.");
    }

    fn write_token(&mut self) {
        self.xml_writer.write_token(
            self.tokenizer.token_type(),
            self.tokenizer.token.as_str(),
            self.tokenizer.int_token,
        );
    }
}
