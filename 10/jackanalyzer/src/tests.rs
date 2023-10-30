#[cfg(test)]
mod tests {
    use crate::tokenizer;

    #[test]
    fn test_tokenizer() {
        {
            let input = "let x = 4;\nlet y = -1;";
            let mut tokenizer = tokenizer::Tokenizer::build(input).unwrap();
            
            println!("Testing input: {}", input); 

            tokenizer.advance();
            assert_eq!(tokenizer.token_type(), tokenizer::TokenType::Keyword);
            assert_eq!(tokenizer.keyword(), tokenizer::KeywordType::Let);

            tokenizer.advance();
            assert_eq!(tokenizer.token_type(), tokenizer::TokenType::Identifier);
            assert_eq!(tokenizer.identifier(), "x");
            
            tokenizer.advance();
            assert_eq!(tokenizer.token_type(), tokenizer::TokenType::Symbol);
            assert_eq!(tokenizer.symbol(), '=');

            tokenizer.advance();
            assert_eq!(tokenizer.token_type(), tokenizer::TokenType::IntConst);
            assert_eq!(tokenizer.int_val(), 4);

            tokenizer.advance();
            assert_eq!(tokenizer.token_type(), tokenizer::TokenType::Symbol);
            assert_eq!(tokenizer.symbol(), ';');

            tokenizer.advance();
            assert_eq!(tokenizer.token_type(), tokenizer::TokenType::Keyword);
            assert_eq!(tokenizer.keyword(), tokenizer::KeywordType::Let);

            tokenizer.advance();
            assert_eq!(tokenizer.token_type(), tokenizer::TokenType::Identifier);
            assert_eq!(tokenizer.identifier(), "y");
            
            tokenizer.advance();
            assert_eq!(tokenizer.token_type(), tokenizer::TokenType::Symbol);
            assert_eq!(tokenizer.symbol(), '=');

            tokenizer.advance();
            assert_eq!(tokenizer.token_type(), tokenizer::TokenType::Symbol);
            assert_eq!(tokenizer.symbol(), '-');

            tokenizer.advance();
            assert_eq!(tokenizer.token_type(), tokenizer::TokenType::IntConst);
            assert_eq!(tokenizer.int_val(), 1);

            tokenizer.advance();
            assert_eq!(tokenizer.token_type(), tokenizer::TokenType::Symbol);
            assert_eq!(tokenizer.symbol(), ';');
        }
    }
}
