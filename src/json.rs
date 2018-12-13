use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Token {
    String,
    Number,
    Comma,
    Colon,
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    EndOfFile,
    Error,
}

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn read(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.read();
        }
    }

    fn read_string(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if !c.is_alphabetic() {
                break;
            }
            self.read();
        }
        self.read();
        Token::String
    }

    fn read_number(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if !c.is_numeric() {
                break;
            }
            self.read();
        }
        Token::Number
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read() {
            Some(':') => Token::Colon,
            Some(',') => Token::Comma,
            Some('{') => Token::ObjectStart,
            Some('}') => Token::ObjectEnd,
            Some('[') => Token::ArrayStart,
            Some(']') => Token::ArrayEnd,
            Some(c) => {
                if c == '"' || c == '\'' {
                    self.read_string()
                } else if c.is_numeric() {
                    self.read_number()
                } else {
                    Token::Error
                }
            }
            None => Token::EndOfFile,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_lex(source: &str, tokens: &[Token]) {
        let mut lexer = Lexer::new(source);
        for tok in tokens {
            let token = lexer.next_token();
            assert_eq!(&token, tok, "found {:?}, expected {:?}", token, tok);
        }
    }

    #[test]
    fn parse_simple() {
        assert_lex(
            r#"{"a": "b", "c": "d"}"#,
            &vec![
                Token::ObjectStart,
                Token::String,
                Token::Colon,
                Token::String,
                Token::Comma,
                Token::String,
                Token::Colon,
                Token::String,
                Token::ObjectEnd,
                Token::EndOfFile,
            ],
        );
    }

    #[test]
    fn parse_nested() {
        assert_lex(
            r#"{"a": [1, 2], "b": {"c": 3}}"#,
            &vec![
                Token::ObjectStart,
                Token::String,
                Token::Colon,
                Token::ArrayStart,
                Token::Number,
                Token::Comma,
                Token::Number,
                Token::ArrayEnd,
                Token::Comma,
                Token::String,
                Token::Colon,
                Token::ObjectStart,
                Token::String,
                Token::Colon,
                Token::Number,
                Token::ObjectEnd,
                Token::ObjectEnd,
                Token::EndOfFile,
            ],
        );
    }
}
