use crate::tokens::{Lexer, Token};
use std::fmt;

#[derive(Debug)]
pub struct Minify {
    minified: Vec<Token>,
}

impl Minify {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let minified = lexer.read_to_end();

        Minify { minified }
    }
}

impl fmt::Display for Minify {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = Vec::new();
        for token in &self.minified {
            match token {
                Token::String(string) => {
                    output.push("\"");
                    output.push(string.as_str());
                    output.push("\"");
                }
                Token::Number(string) => output.push(string.as_str()),
                Token::Null => output.push("null"),
                Token::True => output.push("true"),
                Token::False => output.push("false"),
                Token::Comma => output.push(","),
                Token::Colon => output.push(":"),
                Token::ObjectStart => output.push("{"),
                Token::ObjectEnd => output.push("}"),
                Token::ArrayStart => output.push("["),
                Token::ArrayEnd => output.push("]"),
                Token::EndOfFile => break,
                Token::Error => {
                    eprintln!("Parsing error!");
                    break;
                }
            }
        }

        write!(f, "{}", output.join(""))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        let input = r#"{
  "a": "b",
  "c": "d"
}"#;
        let minified = Minify::new(input);
        assert_eq!(r#"{"a":"b","c":"d"}"#.to_owned(), format!("{}", minified));
    }

    #[test]
    fn medium() {
        let input = r#"{
  "name": "ola nordmann",
  "age": 100,
  "messages": ["hello", "world", "!"]
}"#;
        let minified = Minify::new(input);
        assert_eq!(
            r#"{"name":"ola nordmann","age":100,"messages":["hello","world","!"]}"#,
            format!("{}", minified)
        );
    }
}
