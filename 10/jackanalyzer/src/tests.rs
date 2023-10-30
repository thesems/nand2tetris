#[cfg(test)]
mod tests {
    use crate::tokenizer;

    #[test]
    fn test_tokenizer() {
        {
            let input = "let x = 4;\nlet y = -1;";
            let mut tokenizer = tokenizer::Tokenizer::build(input).unwrap();
            
            println!("Testing input: {}", input);

            let token_types = vec![
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::IntConst,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::IntConst,
                tokenizer::TokenType::Symbol,
            ];

            for token in token_types {
                tokenizer.advance();
                assert_eq!(tokenizer.token_type(), token);
            }
        }
    }
}
