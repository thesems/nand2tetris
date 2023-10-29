use std::{error::Error};

use crate::tokenizer::Tokenizer;

pub struct CompilationEngine {
}

impl CompilationEngine {
    pub fn build(tokenizer: &Tokenizer) -> Result<CompilationEngine, Box<dyn Error>> {
        Ok(CompilationEngine { })
    }

    pub fn compile_class() {}

    pub fn compile_class_var_dec() {}

    pub fn compile_subroutine_dec() {}

    pub fn compile_parameter_list() {}

    pub fn compile_subroutine_body() {}

    pub fn compile_var_dec() {}

    pub fn compile_statements() {}

    pub fn compile_let() {}

    pub fn compile_if() {}

    pub fn compile_while() {}

    pub fn compile_do() {}

    pub fn compile_return() {}

    pub fn compile_expression() {}

    pub fn compile_term() {}

    pub fn compile_expression_list() {}
}
