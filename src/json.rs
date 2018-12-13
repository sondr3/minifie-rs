use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
enum Token<'a> {
    String(String),
    Number(String),
    Comma(&'a str),
    Colon(&'a str),
    ObjectStart(&'a str),
    ObjectEnd(&'a str),
    ArrayStart(&'a str),
    ArrayEnd(&'a str),
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

    fn read_string(&mut self, init: char, inside: bool) -> Token {
        let mut ident = String::new();
        ident.push(init);
        while let Some(c) = self.peek() {
            if !c.is_alphabetic() {
                break;
            }
            ident.push(self.read().expect("Could not parse ident"));
        }
        if inside {
            self.read();
        }
        Token::String(ident)
    }

    fn read_number(&mut self, init: char) -> Token {
        let mut number = String::new();
        number.push(init);
        let allowed = vec!['.', 'e', 'E', '+', '-'];
        while let Some(c) = self.peek() {
            if !c.is_numeric() && !allowed.contains(c) {
                break;
            }
            number.push(self.read().expect("Could not parse number"));
        }
        Token::Number(number)
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read() {
            Some(':') => Token::Colon(":"),
            Some(',') => Token::Comma(","),
            Some('{') => Token::ObjectStart("{"),
            Some('}') => Token::ObjectEnd("}"),
            Some('[') => Token::ArrayStart("["),
            Some(']') => Token::ArrayEnd("]"),
            Some(c) => {
                if c.is_numeric() || c == '-' {
                    self.read_number(c)
                } else if c == '"' || c == '\'' {
                    let c = self.read().unwrap();
                    let token = self.read_string(c, true);
                    token
                } else if c.is_alphabetic() {
                    self.read_string(c, false)
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
            assert_eq!(&token, tok);
        }
    }

    #[test]
    fn parse_simple() {
        assert_lex(
            r#"{"a": "b", "c": "d"}"#,
            &vec![
                Token::ObjectStart("{"),
                Token::String("a".to_string()),
                Token::Colon(":"),
                Token::String("b".to_string()),
                Token::Comma(","),
                Token::String("c".to_string()),
                Token::Colon(":"),
                Token::String("d".to_string()),
                Token::ObjectEnd("}"),
                Token::EndOfFile,
            ],
        );
    }

    #[test]
    fn parse_nested() {
        assert_lex(
            r#"{"a": [1, 2], "b": {"c": 3}}"#,
            &vec![
                Token::ObjectStart("{"),
                Token::String("a".to_string()),
                Token::Colon(":"),
                Token::ArrayStart("["),
                Token::Number("1".to_string()),
                Token::Comma(","),
                Token::Number("2".to_string()),
                Token::ArrayEnd("]"),
                Token::Comma(","),
                Token::String("b".to_string()),
                Token::Colon(":"),
                Token::ObjectStart("{"),
                Token::String("c".to_string()),
                Token::Colon(":"),
                Token::Number("3".to_string()),
                Token::ObjectEnd("}"),
                Token::ObjectEnd("}"),
                Token::EndOfFile,
            ],
        );
    }

    #[test]
    fn grammar() {
        assert_lex("  \t\n\r", &vec![]);
        assert_lex("null", &vec![Token::String("null".to_string())]);
        assert_lex("[]", &vec![Token::ArrayStart("["), Token::ArrayEnd("]")]);
        assert_lex("{}", &vec![Token::ObjectStart("{"), Token::ObjectEnd("}")]);
        assert_lex("15.2", &vec![Token::Number("15.2".to_string())]);
        assert_lex("0.2", &vec![Token::Number("0.2".to_string())]);
        assert_lex("5e9", &vec![Token::Number("5e9".to_string())]);
        assert_lex("-4E-3", &vec![Token::Number("-4E-3".to_string())]);
        assert_lex("true", &vec![Token::String("true".to_string())]);
        assert_lex("false", &vec![Token::String("false".to_string())]);
        assert_lex(r#""""#, &vec![Token::String("\"".to_string())]);
        assert_lex(r#""a""#, &vec![Token::String("a".to_string())]);
        assert_lex(r#""\\""#, &vec![Token::String("\\".to_string())]);
        assert_lex(
            "[null,]",
            &vec![
                Token::ArrayStart("["),
                Token::String("null".to_string()),
                Token::Comma(","),
                Token::ArrayEnd("]"),
            ],
        );
    }
}
