use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String(String),
    Number(String),
    Null,
    True,
    False,
    Comma,
    Colon,
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    EndOfFile,
    Error,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
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

    fn read_ident(&mut self, target: &str) -> bool {
        let mut clone = self.input.clone();
        let len = target.len();
        for c in target.chars() {
            if clone.next() == Some(c) {
                continue;
            } else {
                return false;
            }
        }

        for _ in 0..len {
            self.read();
        }

        true
    }

    fn read_string(&mut self, init: char) -> Token {
        let mut ident = String::new();
        let mut slash = false;
        ident.push(init);
        while let Some(c) = self.peek() {
            if c == &'"' && !slash {
                break;
            }
            if c == &'\\' {
                slash = true;
                ident.push('\\');
                self.read();
                continue;
            }
            if slash && c == &'"' {
                slash = false;
            }
            ident.push(self.read().expect("Could not parse ident"));
        }
        self.read();
        Token::String(ident)
    }

    fn read_number(&mut self, init: char) -> Token {
        let mut number = String::new();
        number.push(init);
        while let Some(c) = self.peek() {
            if c == &',' || c == &']' || c == &'}' || c.is_whitespace() {
                break;
            }
            number.push(self.read().expect("Could not parse number"));
        }
        Token::Number(number)
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read() {
            Some(':') => Token::Colon,
            Some(',') => Token::Comma,
            Some('{') => Token::ObjectStart,
            Some('}') => Token::ObjectEnd,
            Some('[') => Token::ArrayStart,
            Some(']') => Token::ArrayEnd,
            Some(c) => {
                if c == 'n' && self.read_ident("ull") {
                    Token::Null
                } else if c == 't' && self.read_ident("rue") {
                    Token::True
                } else if c == 'f' && self.read_ident("alse") {
                    Token::False
                } else if c == '"' {
                    let c = self.read().unwrap();
                    self.read_string(c)
                } else if c.is_numeric() || c == '-' {
                    self.read_number(c)
                } else {
                    Token::Error
                }
            }
            None => Token::EndOfFile,
        }
    }

    pub fn read_to_end(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.peek() != None {
            tokens.push(self.next_token());
        }

        tokens
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_lex(source: &str, tokens: &[Token]) {
        let mut lexer = Lexer::new(source);
        for tok in tokens {
            let token = lexer.next_token();
            assert_eq!(&token, tok);
        }
    }

    #[test]
    fn parse_simple() {
        assert_lex(
            r#"{"a": "b", "c": "d"}"#,
            &vec![
                Token::ObjectStart,
                Token::String("a".to_string()),
                Token::Colon,
                Token::String("b".to_string()),
                Token::Comma,
                Token::String("c".to_string()),
                Token::Colon,
                Token::String("d".to_string()),
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
                Token::EndOfFile,
            ],
        );
    }

    #[test]
    fn grammar() {
        assert_lex("  \t\n\r", &vec![]);
        assert_lex("null", &vec![Token::Null]);
        assert_lex("[]", &vec![Token::ArrayStart, Token::ArrayEnd]);
        assert_lex("{}", &vec![Token::ObjectStart, Token::ObjectEnd]);
        assert_lex("15.2", &vec![Token::Number("15.2".to_string())]);
        assert_lex("0.2", &vec![Token::Number("0.2".to_string())]);
        assert_lex("5e9", &vec![Token::Number("5e9".to_string())]);
        assert_lex("-4E-3", &vec![Token::Number("-4E-3".to_string())]);
        assert_lex("true", &vec![Token::True]);
        assert_lex("false", &vec![Token::False]);
        assert_lex(r#"" ""#, &vec![Token::String(" ".to_string())]);
        assert_lex(r#""a""#, &vec![Token::String("a".to_string())]);
        // TODO: Make these two work...
        // assert_lex(r#""\"""#, &vec![Token::String("\"".to_string())]);
        // assert_lex(r#""\\""#, &vec![Token::String("\\".to_string())]);
        assert_lex(
            "[null,]",
            &vec![
                Token::ArrayStart,
                Token::Null,
                Token::Comma,
                Token::ArrayEnd,
            ],
        );
    }
}
