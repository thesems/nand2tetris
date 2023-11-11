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
        {
            let input = "do updateFn(param1, param2);";
            let mut tokenizer = tokenizer::Tokenizer::build(input).unwrap();
            
            println!("Testing input: {}", input);

            let token_types = vec![
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Symbol,
            ];

            for token in token_types {
                tokenizer.advance();
                assert_eq!(tokenizer.token_type(), token);
            }
        }
        {
            let input = "function main(myvar) {\nvar int x;\nlet x = 4;\n}\n";
            let mut tokenizer = tokenizer::Tokenizer::build(input).unwrap();
            
            println!("Testing input: {}", input);

            let token_types = vec![
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::IntConst,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Symbol,
            ];

            for token in token_types {
                tokenizer.advance();
                assert_eq!(tokenizer.token_type(), token);
            }
        }
        {
            let input = "class Test {\nfield int myx;\nmethod main(myvar) {\nvar int x;\nlet x = 4;\n}\n}\n";
            let mut tokenizer = tokenizer::Tokenizer::build(input).unwrap();
            
            println!("Testing input: {}", input);

            let token_types = vec![
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Keyword,
                tokenizer::TokenType::Identifier,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::IntConst,
                tokenizer::TokenType::Symbol,
                tokenizer::TokenType::Symbol,
            ];

            for token in token_types {
                tokenizer.advance();
                assert_eq!(tokenizer.token_type(), token);
            }
        }
    }

    #[test]
    fn test_compilation_engine() {
        // let input = "class Test {\nfield int myx;\nmethod main(myvar) {\nvar int x;\nlet x = 4;\n}\n}\n";
        // let mut tokenizer = tokenizer::Tokenizer::build(input).unwrap();
    }
}
