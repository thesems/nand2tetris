use std::{error::Error, fs, io::BufRead};

#[derive(PartialEq, Clone)]
pub enum TokenType {
    Unknown = 0,
    Keyword = 1,
    Symbol = 2,
    Identifier = 3,
    IntConst = 4,
    StringConst = 5,
}

#[derive(PartialEq, Clone)]
pub enum KeywordType {
    Unknown = 0,
    Class = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Int = 5,
    Boolean = 6,
    Char = 7,
    Void = 8,
    Var = 9,
    Static = 10,
    Field = 11,
    Let = 12,
    Do = 13,
    If = 14,
    Else = 15,
    While = 16,
    Return = 17,
    True = 18,
    False = 19,
    Null = 20,
    This = 21,
}

pub struct Tokenizer {
    lines: Vec<String>,
    line_idx: isize,
    char_idx: isize,
    token_type: TokenType,
    keyword_type: KeywordType,
    token: String,
    int_token: u16,
}

impl Tokenizer {
    pub fn build(input: &str) -> Result<Tokenizer, Box<dyn Error>> {
        let contents = fs::read_to_string(input)?;
        let lines = contents
            .split("\n")
            .into_iter()
            .map(|line| line.trim())
            .filter(|line| !line.starts_with("//") && !line.is_empty())
            .map(|line| {
                let res = line.find("//");
                match res {
                    Some(idx) => String::from(line.trim().split_at(idx).0),
                    None => String::from(line.trim()),
                }
            })
            .collect();
        Ok(Tokenizer {
            lines,
            line_idx: -1,
            char_idx: -1,
            token_type: TokenType::Unknown,
            keyword_type: KeywordType::Unknown,
            token: String::from(""),
            int_token: 0,
        })
    }

    pub fn has_more_tokens(&self) -> bool {
        if (self.line_idx as usize) < self.lines.len() {
            return true;
        }

        return false;
    }

    pub fn advance(&mut self) {
        if !self.has_more_tokens() {
            return;
        }

        self.token_type = TokenType::Unknown;
        self.token = String::from("");
        self.keyword_type = KeywordType::Unknown;
        self.int_token = 0;

        let mut token = String::from("");
        loop {
            let res = self.lines[self.line_idx as usize]
                .chars()
                .nth(self.char_idx as usize);

            let ch = match res {
                Some(c) => c,
                None => {
                    // if we ran out of chars in this line, exit loop
                    println!("Prematurely exited the building token loop.");
                    break;
                }
            };

            match ch {
                '{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' | ';' | '+' | '-' | '*' | '/'
                | '&' | '|' | '<' | '>' | '=' | '~' => {
                    if token != "" {
                        break;
                    } else {
                        // add all other symbols
                        self.token_type = TokenType::Symbol;
                        self.token = String::from(ch);
                    }
                }
                ' ' => {
                    break;
                }
                _ => {
                    token.push(
                        self.lines[self.line_idx as usize]
                            .chars()
                            .nth(self.char_idx as usize)
                            .unwrap(),
                    );
                }
            }
        }
        
        // advance to next character
        if (self.char_idx as usize) == self.lines[self.line_idx as usize].len() {
            self.line_idx = self.line_idx + 1;
            self.char_idx = 0;
        } else {
            self.char_idx = self.char_idx + 1;
        }

        // determine the token type
        self.token = token;
        self.determine_token_type();
    }

    fn determine_token_type(&mut self) {
        // check if keyword
        self.keyword_type = match self.token.as_str() {
            "class" => KeywordType::Class,
            "constructor" => KeywordType::Constructor,
            "function" => KeywordType::Function,
            "method" => KeywordType::Method,
            "field" => KeywordType::Field,
            "static" => KeywordType::Static,
            "var" => KeywordType::Var,
            "int" => KeywordType::Int,
            "char" => KeywordType::Char,
            "boolean" => KeywordType::Boolean,
            "void" => KeywordType::Void,
            "true" => KeywordType::True,
            "false" => KeywordType::False,
            "null" => KeywordType::Null,
            "this" => KeywordType::This,
            "let" => KeywordType::Let,
            "do" => KeywordType::Do,
            "if" => KeywordType::If,
            "else" => KeywordType::Else,
            "while" => KeywordType::While,
            "return" => KeywordType::Return,
            _ => KeywordType::Unknown,
        };

        if self.keyword_type != KeywordType::Unknown {
            self.token_type = TokenType::Keyword;
            return;
        }

        // check if integerConstant
        let num = self.token.parse::<i32>().unwrap_or_else(|_| -1);
        if num != -1 {
            self.token_type = TokenType::IntConst;
            self.int_token = num as u16;
            return;
        }

        // check if stringConstant
        if self.token.starts_with("\"") && self.token.ends_with("\"") {
            self.token_type = TokenType::StringConst;
            return;
        }

        // check if identifier
        self.token_type = TokenType::Identifier;
    }

    pub fn token_type(&self) -> TokenType {
        return self.token_type.clone();
    }

    pub fn keyword(&self) -> KeywordType {
        if self.token_type() != TokenType::Keyword {
            panic!("keyword() can only be called if the current token is of type keyword!");
        }
        return self.keyword_type.clone();
    }

    pub fn symbol(&self) -> char {
        if self.token_type() != TokenType::Symbol {
            panic!("symbol() can only be called if the current token is of type symbol!");
        }
        return self.token.chars().nth(0).unwrap_or_else(|| {
            panic!("symbol() has been called, however no symbol is available as a token!");
        });
    }

    pub fn identifier(&self) -> String {
        if self.token_type() != TokenType::Identifier {
            panic!("identifier() can only be called if the current token is of type identifier!");
        }
        return self.token.clone();
    }

    pub fn int_val(&self) -> u16 {
        return self.int_token;
    }

    pub fn string_val(&self) -> String {
        let mut token = self.token.chars();
        token.next();
        token.next_back();
        return String::from(token.as_str());
    }
}
