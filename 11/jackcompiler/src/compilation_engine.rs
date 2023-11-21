use std::error::Error;

use crate::{
    symbol_table::{KindType, SymbolTable},
    tokenizer::{KeywordType, TokenType, Tokenizer},
    vm_writer::{Operation, Segment, VmWriter},
    xml_writer::XmlWriter,
};

pub struct CompilationEngine<'a> {
    tokenizer: &'a mut Tokenizer,
    xml_writer: &'a mut XmlWriter,
    vm_writer: &'a mut VmWriter,
    class_symtab: SymbolTable,
    func_symtab: SymbolTable,
    if_counter: u16,
    while_counter: u16,
    current_class: String,
    current_func: String,
    current_func_type: String,
}

impl<'a> CompilationEngine<'a> {
    pub fn build(
        tokenizer: &'a mut Tokenizer,
        xml_writer: &'a mut XmlWriter,
        vm_writer: &'a mut VmWriter,
    ) -> Result<CompilationEngine<'a>, Box<dyn Error>> {
        tokenizer.reset();
        Ok(CompilationEngine {
            tokenizer,
            xml_writer,
            vm_writer,
            class_symtab: SymbolTable::build(),
            func_symtab: SymbolTable::build(),
            if_counter: 0,
            while_counter: 0,
            current_class: String::from(""),
            current_func: String::from(""),
            current_func_type: String::from(""),
        })
    }

    fn kind_to_segment(kind_type: &KindType) -> Segment {
        return match kind_type {
            KindType::VAR => Segment::LOCAL,
            KindType::ARG => Segment::ARGUMENT,
            KindType::FIELD => Segment::THIS,
            KindType::STATIC => Segment::STATIC,
        };
    }

    fn print_compile_error(&self, message: &str) {
        let token = match self.tokenizer.token_type() {
            TokenType::IntConst => self.tokenizer.int_token.to_string(),
            _ => self.tokenizer.token.clone(),
        };

        println!(
            "Compile error: '{}' on line {}. Received token: {}",
            message, self.tokenizer.line_idx, token
        );
        println!(
            "Lines:\n    {}\n    {}",
            self.tokenizer.lines[self.tokenizer.line_idx - 1],
            self.tokenizer.lines[self.tokenizer.line_idx]
        );
        panic!("");
    }

    pub fn compile_class(&mut self) {
        self.xml_writer.write_full_tag("<class>");
        
        // Clear class symbol table
        self.class_symtab.start_subroutine();

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Keyword || self.tokenizer.token != "class" {
            self.print_compile_error("Expected keyword class.");
        }
        self.write_token();

        self._compile_identifier(
            "Expected a class identifier name after keyword class.",
            "",
            &KindType::ARG,
            false,
            false,
        );

        self.current_class = self.tokenizer.token.clone();

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "{" {
            self.print_compile_error("Expected a symbol {{.");
        }
        self.write_token();

        self.tokenizer.advance();
        while !(self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "}") {
            if self.tokenizer.token_type() != TokenType::Keyword {
                self.print_compile_error("Expected one of the following keywords: static, field, constructor, method, function.");
            }
            match self.tokenizer.token.as_str() {
            "static" | "field" => {
                self.compile_class_var_dec();
            }
            "constructor" | "function" | "method" => {
                self.compile_subroutine_dec();
            }
            _ => self.print_compile_error("Expected one of the following keywords: static, field, constructor, method, function.")
        };
            self.tokenizer.advance();
        }

        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "}" {
            self.print_compile_error("Expected a symbol }.");
        }
        self.write_token();

        self.xml_writer.write_full_tag("</class>");
    }

    pub fn compile_class_var_dec(&mut self) {
        self.xml_writer.write_full_tag("<classVarDec>");

        // static or field
        self.write_token();
        let kind_type = match self.tokenizer.token.as_str() {
            "static" => KindType::STATIC,
            "field" => KindType::FIELD,
            _ => panic!("Should not happen!"),
        };

        // keyword: variable type
        self.tokenizer.advance();

        let allowed_keywords = vec!["int", "char", "boolean"];
        if self.tokenizer.token_type() != TokenType::Identifier
            && self.tokenizer.token_type() != TokenType::Keyword
            && !allowed_keywords.contains(&&self.tokenizer.token.as_str())
        {
            self.print_compile_error("Expected a variable type.");
        }
        self.write_token();

        self._compile_identifier(
            "Expected a variable identifier name after variable type.",
            self.tokenizer.token.clone().as_str(),
            &kind_type,
            true,
            true,
        );

        loop {
            self.tokenizer.advance();
            if self.tokenizer.token_type() != TokenType::Symbol {
                self.print_compile_error("Expected a symbol.");
            }

            if self.tokenizer.token == ";" {
                self.write_token();
                break;
            }

            if self.tokenizer.token != "," {
                self.print_compile_error("Expected a comma.");
            }

            self.write_token();

            // varName
            self._compile_identifier(
                "Expected a variable identifier name after ',' comma symbol.",
                self.tokenizer.token.clone().as_str(),
                &kind_type,
                true,
                true,
            );
        }

        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != ";" {
            self.print_compile_error("Expected an symbol.");
        }

        self.xml_writer.write_full_tag("</classVarDec>");
    }

    pub fn compile_subroutine_dec(&mut self) {
        self.xml_writer.write_full_tag("<subroutineDec>");
        
        // Clear function symbol table
        self.func_symtab.start_subroutine();

        let allowed_keywords = vec!["constructor", "function", "method"];
        if self.tokenizer.token_type() != TokenType::Keyword
            || !allowed_keywords.contains(&&self.tokenizer.token.as_str())
        {
            self.print_compile_error("Expected a keyword: constructor, function or method.");
        }
        self.write_token();
        self.current_func_type = self.tokenizer.token.clone();

        self.tokenizer.advance();
        let allowed_keywords = vec!["void", "int", "char", "boolean"];
        if self.tokenizer.token_type() != TokenType::Identifier
            && self.tokenizer.token_type() != TokenType::Keyword
            && !allowed_keywords.contains(&&self.tokenizer.token.as_str())
        {
            self.print_compile_error("Expected a variable type.");
        }
        self.write_token();

        // subroutineName
        self._compile_identifier(
            "Expected a subroutine identifier name.",
            "",
            &KindType::ARG,
            false,
            false,
        );
        self.current_func = self.tokenizer.token.clone();

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "(" {
            self.print_compile_error("Expected an opening bracket '('.");
        }
        self.write_token();

        self.compile_parameter_list();

        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != ")" {
            self.print_compile_error("Expected a closing bracket.");
        }
        self.write_token();

        self.compile_subroutine_body();
        self.xml_writer.write_full_tag("</subroutineDec>");
    }

    pub fn compile_parameter_list(&mut self) {
        self.xml_writer.write_full_tag("<parameterList>");

        self.tokenizer.advance();
        while !(self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == ")") {
            let allowed_keywords = vec!["void", "int", "char", "boolean"];
            if self.tokenizer.token_type() != TokenType::Identifier
                && self.tokenizer.token_type() != TokenType::Keyword
                && !allowed_keywords.contains(&&self.tokenizer.token.as_str())
            {
                self.print_compile_error("Expected a variable type.");
            }
            self.write_token();

            self._compile_identifier(
                "Expected a parameter name identifier.",
                self.tokenizer.token.clone().as_str(),
                &KindType::ARG,
                true,
                true,
            );

            self.tokenizer.advance();
            if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "," {
                self.write_token();
                self.tokenizer.advance();
            }
        }

        self.xml_writer.write_full_tag("</parameterList>");
    }

    pub fn compile_subroutine_body(&mut self) {
        self.xml_writer.write_full_tag("<subroutineBody>");
        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "{" {
            self.print_compile_error(
                "Expected the function body to start with an opening curvy bracket '{{'.",
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

        // VM-Code Generator
        let local_count = self.func_symtab.var_count(&KindType::VAR);
        let mut full_func_name = self.current_class.clone();
        full_func_name.push('.');
        full_func_name.push_str(self.current_func.as_str());
        self.vm_writer
            .write_function(full_func_name.as_str(), local_count);

        if self.current_func_type == "constructor" {
            let n = self.class_symtab.var_count(&KindType::VAR);
            self.vm_writer.write_push(Segment::CONSTANT, n);
            self.vm_writer.write_call("Memory.alloc", 1);
            self.vm_writer.write_pop(Segment::POINTER, 0);
        } else if self.current_func_type == "method" {
            self.vm_writer.write_push(Segment::ARGUMENT, 0);
            self.vm_writer.write_pop(Segment::POINTER, 0);
        }

        self.compile_statements();

        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "}" {
            self.print_compile_error(
                "Expected the function body to start with a closed curly bracket '}}'.",
            );
        }
        self.write_token();
        self.xml_writer.write_full_tag("</subroutineBody>");
    }

    pub fn compile_var_dec(&mut self) {
        self.xml_writer.write_full_tag("<varDec>");
        self.write_token();

        self.tokenizer.advance();
        let allowed_keywords = vec!["int", "char", "boolean"];

        if self.tokenizer.token_type() != TokenType::Identifier
            && self.tokenizer.token_type() != TokenType::Keyword
            && !allowed_keywords.contains(&&self.tokenizer.token.as_str())
        {
            self.print_compile_error("Expected a variable type.");
        }
        self.write_token();

        let typ = self.tokenizer.token.clone();
        self._compile_identifier(
            "Expected a variable name identifier.",
            typ.as_str(),
            &KindType::VAR,
            true,
            true,
        );

        loop {
            self.tokenizer.advance();
            if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == ";" {
                self.write_token();
                break;
            }

            if !(self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == ",") {
                self.print_compile_error("Expected a comma after a variable name identifier.");
            }
            self.write_token();

            self._compile_identifier(
                "Expected a variable name identifier.",
                typ.as_str(),
                &KindType::VAR,
                true,
                true,
            );
        }
        self.xml_writer.write_full_tag("</varDec>");
    }

    pub fn compile_statements(&mut self) {
        self.xml_writer.write_full_tag("<statements>");
        loop {
            if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "}" {
                break;
            } else if self.tokenizer.token_type() != TokenType::Keyword {
                self.print_compile_error(
                    "A statement must include a keyword: let, if, while, do, return.",
                );
            }

            match self.tokenizer.token.as_str() {
                "let" => {
                    self.compile_let();
                    self.tokenizer.advance();
                }
                "if" => {
                    self.compile_if();
                }
                "while" => {
                    self.compile_while();
                    self.tokenizer.advance();
                }
                "do" => {
                    self.compile_do();
                    self.tokenizer.advance();
                }
                "return" => {
                    self.compile_return();
                    self.tokenizer.advance();
                }
                _ => {
                    self.print_compile_error("Unrecongnized statement keyword.");
                }
            }
        }
        self.xml_writer.write_full_tag("</statements>");
    }

    pub fn compile_let(&mut self) {
        self.xml_writer.write_full_tag("<letStatement>");
        self.write_token();

        self.tokenizer.advance();

        let mut is_array = false;
        let typ = self
            .class_symtab
            .type_of(self.tokenizer.token.as_str())
            .unwrap_or_else(|| {
                return self
                    .func_symtab
                    .type_of(self.tokenizer.token.as_str())
                    .unwrap();
            });
        let index = self
            .class_symtab
            .index_of(self.tokenizer.token.as_str())
            .unwrap_or_else(|| {
                return self
                    .func_symtab
                    .index_of(self.tokenizer.token.as_str())
                    .unwrap();
            });
        let kind_type = self
            .class_symtab
            .kind_of(self.tokenizer.token.as_str())
            .unwrap_or_else(|| {
                return self
                    .func_symtab
                    .kind_of(self.tokenizer.token.as_str())
                    .unwrap();
            });

        self._compile_identifier(
            "Expected an existing variable name.",
            String::from(typ).as_str(),
            &kind_type,
            true,
            false,
        );

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol {
            self.print_compile_error("Expected a symbol '=' or array deferencing '['.");
        }

        if self.tokenizer.token == "[" {
            self.write_token();

            self.tokenizer.advance();
            self.compile_expression();

            // VM-Code Generator
            let segment = CompilationEngine::kind_to_segment(&kind_type);
            self.vm_writer.write_push(segment, index);
            self.vm_writer.write_arithmetic(Operation::ADD);
            is_array = true;

            if self.tokenizer.token != "]" || self.tokenizer.token_type() != TokenType::Symbol {
                self.print_compile_error(
                    "Expected an closing square bracket symbol after deferencing expression.",
                );
            }
            self.write_token();

            self.tokenizer.advance();
            if self.tokenizer.token != "=" || self.tokenizer.token_type() != TokenType::Symbol {
                self.print_compile_error("Expected an equal '=' symbol after variable name.");
            }
            self.write_token();

            // TODO: handle let arrays indexing compilation
        } else if self.tokenizer.token == "=" {
            self.write_token();
        } else {
            self.print_compile_error("Expected a symbol '=' or array deferencing '['.");
        }

        self.tokenizer.advance();
        self.compile_expression();

        // VM-Code Generation
        if is_array {
            self.vm_writer.write_pop(Segment::TEMP, 0);
            self.vm_writer.write_pop(Segment::POINTER, 1);
            self.vm_writer.write_push(Segment::TEMP, 0);
            self.vm_writer.write_pop(Segment::THAT, 0);
        } else {
            let segment = CompilationEngine::kind_to_segment(&kind_type);
            self.vm_writer.write_pop(segment, index);
        }

        if self.tokenizer.token != ";" || self.tokenizer.token_type() != TokenType::Symbol {
            self.print_compile_error("ERROR: expected a semi-colon ';' symbol, but received: {}");
        }
        self.write_token();

        self.xml_writer.write_full_tag("</letStatement>");
    }

    pub fn compile_if(&mut self) {
        self.xml_writer.write_full_tag("<ifStatement>");
        self.write_token();

        self.tokenizer.advance();
        if self.tokenizer.token != "(" || self.tokenizer.token_type() != TokenType::Symbol {
            self.print_compile_error("Expected opening bracket after keyword if!");
        }
        self.write_token();

        self.tokenizer.advance();
        self.compile_expression();

        if self.tokenizer.token != ")" || self.tokenizer.token_type() != TokenType::Symbol {
            self.print_compile_error("Expected closing bracket after keyword if!");
        }
        self.write_token();

        // VM-Code Generation
        let iff_label = format!("IF_FALSE{}", self.if_counter);
        let end_label = format!("IF_END{}", self.if_counter);
        self.if_counter = self.if_counter + 1;

        self.vm_writer.write_arithmetic(Operation::NOT);
        self.vm_writer.write_if(iff_label.as_str());

        self.tokenizer.advance();
        if self.tokenizer.token != "{" || self.tokenizer.token_type() != TokenType::Symbol {
            self.print_compile_error("Expected an opening curvy bracket after keyword if!");
        }
        self.write_token();

        self.tokenizer.advance();
        self.compile_statements();

        if self.tokenizer.token != "}" || self.tokenizer.token_type() != TokenType::Symbol {
            self.print_compile_error("Expected an closing curvy bracket after keyword if.");
        }
        self.write_token();
        
        self.vm_writer.write_goto(end_label.as_str());
        self.vm_writer.write_label(iff_label.as_str());

        self.tokenizer.advance();
        if self.tokenizer.token_type() == TokenType::Keyword
            && self.tokenizer.keyword() == KeywordType::Else
        {
            self.write_token();

            self.tokenizer.advance();
            if self.tokenizer.token != "{" || self.tokenizer.token_type() != TokenType::Symbol {
                self.print_compile_error("Expected an opening curvy bracket after keyword if!");
            }
            self.write_token();

            self.tokenizer.advance();
            self.compile_statements();

            if self.tokenizer.token != "}" || self.tokenizer.token_type() != TokenType::Symbol {
                self.print_compile_error("Expected an closing curvy bracket after keyword if.");
            }
            self.write_token();
            self.tokenizer.advance();
        }

        // VM-Code Generation
        self.vm_writer.write_label(end_label.as_str());

        self.xml_writer.write_full_tag("</ifStatement>");
    }

    pub fn compile_while(&mut self) {
        self.xml_writer.write_full_tag("<whileStatement>");
        self.write_token();

        self.tokenizer.advance();
        if self.tokenizer.token != "(" || self.tokenizer.token_type() != TokenType::Symbol {
            self.print_compile_error("Expected opening bracket after keyword while!");
        }
        self.write_token();

        // VM-Code Generation
        let l1 = format!("WHILE_EXP{}", self.while_counter);
        let l2 = format!("WHILE_END{}", self.while_counter);
        self.while_counter = self.while_counter + 1;
        self.vm_writer.write_label(l1.as_str());

        self.tokenizer.advance();
        self.compile_expression();

        // VM-Code Generation
        self.vm_writer.write_arithmetic(Operation::NOT);
        self.vm_writer.write_if(l2.as_str());

        if self.tokenizer.token != ")" || self.tokenizer.token_type() != TokenType::Symbol {
            self.print_compile_error("Expected closing bracket after keyword while!");
        }
        self.write_token();

        self.tokenizer.advance();
        if self.tokenizer.token != "{" || self.tokenizer.token_type() != TokenType::Symbol {
            self.print_compile_error("Expected an opening curvy bracket after keyword while!");
        }
        self.write_token();

        self.tokenizer.advance();
        self.compile_statements();

        // VM-Code Generation
        self.vm_writer.write_goto(l1.as_str());

        if self.tokenizer.token != "}" || self.tokenizer.token_type() != TokenType::Symbol {
            self.print_compile_error("Expected an closing curvy bracket after keyword while.");
        }
        self.write_token();

        // VM-Code Generation
        self.vm_writer.write_label(l2.as_str());
        self.xml_writer.write_full_tag("</whileStatement>");
    }

    pub fn compile_do(&mut self) {
        self.xml_writer.write_full_tag("<doStatement>");
        self.write_token();

        self._compile_identifier(
            "Expected a subroutine name identifier.",
            "",
            &KindType::ARG,
            false,
            false,
        );

        let mut func_name = self.tokenizer.token.clone();
        self.tokenizer.advance();

        if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "." {
            self.write_token();

            self._compile_identifier(
                "Expected a subroutine name identifier after dot '.' symbol.",
                "",
                &KindType::ARG,
                false,
                false,
            );

            func_name.push('.');
            func_name.push_str(self.tokenizer.token.as_str());
            self.tokenizer.advance();
        }

        if self.tokenizer.token_type() != TokenType::Symbol && self.tokenizer.token != "(" {
            self.print_compile_error("Expected an opening bracket after expression list.");
        }
        self.write_token();
        self.tokenizer.advance();

        let n = self.compile_expression_list();

        // TODO: rewrite whole function to be handled in compile_expression?
        // VM-Code Generator
        self.vm_writer.write_call(func_name.as_str(), n);
        self.vm_writer.write_pop(Segment::TEMP, 0);

        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != ")" {
            self.print_compile_error("Expected a closing bracket after expression list.");
        }
        self.write_token();

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != ";" {
            self.print_compile_error("Expected a semi-colon after do methodcall(...)");
        }
        self.write_token();

        self.xml_writer.write_full_tag("</doStatement>");
    }

    pub fn compile_return(&mut self) {
        self.xml_writer.write_full_tag("<returnStatement>");
        self.write_token();

        let mut has_return_value = false;

        self.tokenizer.advance();
        if self.tokenizer.token_type() != TokenType::Symbol {
            self.compile_expression();

            if self.tokenizer.token_type() != TokenType::Symbol {
                self.print_compile_error("Expected a semi-colon.");
            }

            has_return_value = true;
        }

        if self.tokenizer.token != ";" {
            self.print_compile_error("Expected a symbol ';' but received: {}.");
        }
        self.write_token();

        // VM-Code Generator
        if !has_return_value {
            self.vm_writer.write_push(Segment::CONSTANT, 0);
        }
        self.vm_writer.write_return();

        self.xml_writer.write_full_tag("</returnStatement>");
    }

    pub fn compile_expression(&mut self) {
        self.xml_writer.write_full_tag("<expression>");
        self.compile_term();

        let symbols = vec!["+", "-", "*", "/", "&", "|", "<", ">", "="];
        if self.tokenizer.token_type() == TokenType::Symbol
            && symbols.contains(&&self.tokenizer.token.as_str())
        {
            let symbol = self.tokenizer.token.clone();
            self.write_token();

            self.tokenizer.advance();
            self.compile_term();

            // VM-Code Generator
            let op = match symbol.as_str() {
                "+" => Operation::ADD,
                "-" => Operation::SUB,
                "&" => Operation::AND,
                "|" => Operation::OR,
                "=" => Operation::EQ,
                ">" => Operation::GT,
                "<" => Operation::LT,
                "*" => Operation::MULT,
                "/" => Operation::DIV,
                _ => panic!("Should not never reach!"),
            };
            self.vm_writer.write_arithmetic(op);
        }

        self.xml_writer.write_full_tag("</expression>");
    }

    pub fn compile_term(&mut self) {
        self.xml_writer.write_full_tag("<term>");

        let keyword_constants = vec!["true", "false", "null", "this"];
        let unary_op = vec!["-", "~"];
        let mut func_name = String::from(""); // in case it is a function call

        let safe_exit = |s: &mut CompilationEngine| {
            s.xml_writer.write_full_tag("</term>");
            s.tokenizer.advance();
        };

        if self.tokenizer.token_type() == TokenType::IntConst {
            self.write_token();
            // VM-Code Generator
            self.vm_writer
                .write_push(Segment::CONSTANT, self.tokenizer.int_token);
            safe_exit(self);
            return;
        } else if self.tokenizer.token_type() == TokenType::Keyword {
            if !keyword_constants.contains(&&self.tokenizer.token.as_str()) {
                self.print_compile_error("Non-allowed keyword.");
            }
            // VM-Code Generator
            match self.tokenizer.keyword() {
                KeywordType::True => {
                    self.vm_writer.write_push(Segment::CONSTANT, 0);
                    self.vm_writer.write_arithmetic(Operation::NOT);
                }
                KeywordType::False | KeywordType::Null => {
                    self.vm_writer.write_push(Segment::CONSTANT, 0);
                }
                KeywordType::This => {
                    self.vm_writer.write_push(Segment::POINTER, 0);
                }
                _ => panic!("Should not reach this ever!"), 
            };

            self.write_token();
            safe_exit(self);
            return;
        } else if self.tokenizer.token_type() == TokenType::StringConst {
            // VM-Code Generation
            let size = self.tokenizer.token.len() as u16;
            self.vm_writer.write_push(Segment::CONSTANT, size);
            self.vm_writer.write_call("String.new", 1);
            self.vm_writer.write_pop(Segment::TEMP, 5); // TODO: careful not to reuse temp multiple

            for c in self.tokenizer.token.chars() {
                self.vm_writer.write_push(Segment::TEMP, 5); // TODO: careful not to reuse temp multiple
                self.vm_writer.write_push(Segment::CONSTANT, c as u16);
                self.vm_writer.write_call("String.appendChar", 2);
                self.vm_writer.write_pop(Segment::TEMP, 5);
            }
                
            self.vm_writer.write_push(Segment::TEMP, 5);
            self.write_token();
            safe_exit(self);
            return;
        } else if self.tokenizer.token_type() == TokenType::Symbol
            && unary_op.contains(&&self.tokenizer.token.as_str())
        {
            let op = match self.tokenizer.token.as_str() {
                "~" => Operation::NOT,
                "-" => Operation::NEG,
                _ => panic!("Should not happen ever!"),
            };
            self.write_token();
            self.tokenizer.advance();
            self.compile_term();

            // VM-Code Generator
            self.vm_writer.write_arithmetic(op);

            self.xml_writer.write_full_tag("</term>");
            return;
        } else if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "(" {
            self.write_token();
            self.tokenizer.advance();
            self.compile_expression();

            if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != ")" {
                self.print_compile_error("Expected a closing bracket after expression list.");
            }
            self.write_token();
            safe_exit(self);
            return;
        } else if self.tokenizer.token_type() == TokenType::Identifier {
            let mut typ = self.func_symtab.type_of(self.tokenizer.token.as_str());
            if typ.is_none() {
                typ = self.class_symtab.type_of(self.tokenizer.token.as_str());
            }

            if typ.is_some() {
                let kind_type = self
                    .func_symtab
                    .kind_of(self.tokenizer.token.as_str())
                    .unwrap();
                self._compile_identifier(
                    "Expected an existing variable name.",
                    String::from(typ.unwrap()).as_str(),
                    &kind_type,
                    true,
                    false,
                );

                // VM-Code Generator
                let segment = Self::kind_to_segment(&kind_type);
                let index = self
                    .func_symtab
                    .index_of(self.tokenizer.token.as_str())
                    .unwrap();
                self.vm_writer.write_push(segment, index);
            } else {
                // function/method/constructor
                self.write_token();
                func_name = self.tokenizer.token.clone();
            }
        } else {
            self.print_compile_error("Expected an identifier.");
        }

        self.tokenizer.advance();

        if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "[" {
            self.write_token();
            self.tokenizer.advance();
            self.compile_expression();

            // VM-Code Generator
            self.vm_writer.write_arithmetic(Operation::ADD);
            self.vm_writer.write_pop(Segment::POINTER, 1);
            self.vm_writer.write_push(Segment::THAT, 0);

            if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "]" {
                self.print_compile_error(
                    format!(
                        "Expected a closing square bracket after expression, but received: {}",
                        self.tokenizer.token
                    )
                    .as_str(),
                );
            }
            self.write_token();
        } else if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "(" {
            self.write_token();
            self.tokenizer.advance();
            self.compile_expression_list();

            if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != ")" {
                self.print_compile_error("Expected a closing bracket after expression list.");
            }
            self.write_token();
        } else if self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == "." {
            self.write_token();

            self._compile_identifier(
                "Expected a subroutine name identifier after dot '.' symbol.",
                "",
                &KindType::ARG,
                false,
                false,
            );

            func_name.push('.');
            func_name.push_str(self.tokenizer.token.as_str());

            self.tokenizer.advance();
            if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "(" {
                self.print_compile_error("Expected an opening bracket after expression list.");
            }
            self.write_token();

            self.tokenizer.advance();
            let n = self.compile_expression_list();

            if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != ")" {
                self.print_compile_error("Expected a closing bracket after expression list.");
            }
            self.write_token();

            // VM-Code Generation
            self.vm_writer.write_call(func_name.as_str(), n);
        } else {
            self.xml_writer.write_full_tag("</term>");
            return;
        }

        self.tokenizer.advance();
        self.xml_writer.write_full_tag("</term>");
    }

    pub fn compile_expression_list(&mut self) -> u16 {
        self.xml_writer.write_full_tag("<expressionList>");

        let mut n = 0;
        while !(self.tokenizer.token_type() == TokenType::Symbol && self.tokenizer.token == ")") {
            self.compile_expression();
            n = n + 1;

            if self.tokenizer.token_type() != TokenType::Symbol || self.tokenizer.token != "," {
                break;
            }
            self.write_token();

            self.tokenizer.advance();
        }

        self.xml_writer.write_full_tag("</expressionList>");
        return n;
    }

    fn _compile_identifier(
        &mut self,
        message: &str,
        typ: &str,
        kind_type: &KindType,
        complex_identifier: bool,
        declare: bool,
    ) {
        if !(complex_identifier && !declare) {
            self.tokenizer.advance();
        }

        if self.tokenizer.token_type() != TokenType::Identifier {
            self.print_compile_error(format!("{}", message).as_str());
        }

        if declare {
            match *kind_type {
                KindType::VAR | KindType::ARG => {
                    self.func_symtab
                        .define(self.tokenizer.token.as_str(), typ, kind_type.clone());
                }
                KindType::FIELD | KindType::STATIC => {
                    self.class_symtab
                        .define(self.tokenizer.token.as_str(), typ, kind_type.clone());
                }
            };
        }

        if complex_identifier {
            let index = self
                .func_symtab
                .index_of(self.tokenizer.token.as_str())
                .unwrap();

            self.xml_writer.write_full_tag("<identifier>");
            self.xml_writer.write_full_tag("<name>");
            self.xml_writer
                .write_full_tag(format!("{}", self.tokenizer.token).as_str());
            self.xml_writer.write_full_tag("</name>");
            self.xml_writer.write_full_tag("<typ>");
            self.xml_writer.write_full_tag(format!("{}", typ).as_str());
            self.xml_writer.write_full_tag("</typ>");
            self.xml_writer.write_full_tag("<kind>");
            self.xml_writer
                .write_full_tag(format!("{}", kind_type).as_str());
            self.xml_writer.write_full_tag("</kind>");
            self.xml_writer.write_full_tag("<index>");
            self.xml_writer
                .write_full_tag(format!("{}", index).as_str());
            self.xml_writer.write_full_tag("</index>");
            self.xml_writer.write_full_tag("</identifier>");
        } else {
            self.write_token();
        }
    }

    fn _compile_subroutine_call(&mut self) {
        self._compile_identifier(
            "ERROR: expected a identifier.",
            "",
            &KindType::ARG,
            false,
            false,
        );
    }

    fn write_token(&mut self) {
        self.xml_writer.write_token(
            self.tokenizer.token_type(),
            self.tokenizer.token.as_str(),
            self.tokenizer.int_token,
        );
    }
}
