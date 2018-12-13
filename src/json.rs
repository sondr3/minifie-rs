use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Token {
    String(String),
    Number(String),
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
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if !c.is_alphabetic() {
                break;
            }
            ident.push(self.read().unwrap());
        }
        self.read();
        Token::String(ident)
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        while let Some(c) = self.peek() {
            if !c.is_numeric() {
                break;
            }
            number.push(self.read().unwrap());
        }
        Token::Number(number)
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read() {
            Some(':') => Token::Colon,
            Some(',') => Token::Comma,
            Some('{') => Token::ObjectStart,
            Some('}') => Token::ObjectEnd,
            Some('[') => Token::ArrayStart,
            Some(']') => Token::ObjectEnd,
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

    #[test]
    fn parse() {
        let input = r#"{"a": "b", "c": "d"}"#;
        let expected = vec![
            Token::ObjectStart,
            Token::String("a".to_string()),
            Token::Colon,
            Token::String("b".to_string()),
            Token::Comma,
            Token::String("c".to_string()),
            Token::Colon,
            Token::String("d".to_string()),
            Token::ObjectEnd,
        ];
        let mut lexer = Lexer::new(input);
        for t in expected {
            let token = lexer.next_token();
            assert_eq!(token, t);
        }
    }

    #[test]
    fn parse_2() {
        let input = r#"{"a": [1, 2], "b": {"c": 3}}"#;
        let expected = vec![
            Token::ObjectStart,
            Token::String("a".to_string()),
            Token::Colon,
            Token::ArrayStart,
            Token::Number("1".to_string()),
            Token::Comma,
            Token::Number("2".to_string()),
            Token::ArrayEnd,
            Token::Comma,
            Token::String("b".to_string()),
            Token::Colon,
            Token::ObjectStart,
            Token::String("c".to_string()),
            Token::Colon,
            Token::Number("3".to_string()),
            Token::ObjectEnd,
            Token::ObjectEnd,
        ];
    }
}
