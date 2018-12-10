use ::logos::{Lexer, Logos, Slice, Source};

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[end]
    End,

    #[regex = "\"([^\"\\\\]|\\\\.)*\""]
    #[regex = "'([^'\\\\]|\\\\.)*'"]
    String,

    #[token = "~="]
    IncludeMatch,
    #[token = "|="]
    DashMatch,
    #[token = "^="]
    PrefixMatch,
    #[token = "$="]
    SuffixMatch,
    #[token = "*="]
    SubstringMatch,
    #[token = "||"]
    ColumnToken,
    #[token = "<!--"]
    CDO,
    #[token = "-->"]
    CDC,

    #[regex = "[-+0-9.eE+-]+"]
    Number,
    #[regex = "[-a-zA-Z_][a-zA-Z0-9_-]*"]
    Ident,

    #[token = "@"]
    At,
    #[token = "#"]
    Hash,
    #[token = "$"]
    Dollar,
    #[token = "%"]
    Percentage,
    #[token = ","]
    Comma,
    #[token = "."]
    Period,
    #[token = "*"]
    Asterisk,
    #[token = ";"]
    Semicolon,
    #[token = ":"]
    Colon,

    #[token = "("]
    LeftParen,
    #[token = ")"]
    RightParen,
    #[token = "{"]
    LeftCurlyBracket,
    #[token = "}"]
    RightCurlyBracket,
    #[token = "["]
    LeftBracket,
    #[token = "]"]
    RightBracket,

    #[token = "/*"]
    #[callback = "ignore_comments"]
    #[error]
    UnexpectedToken,
}

// https://github.com/paritytech/lunarity/blob/master/lexer/src/token.rs
fn ignore_comments<'source, Src>(lex: &mut Lexer<Token, Src>)
where
    Src: Source<'source>,
{
    use logos::internal::LexerInternal;

    if lex.slice().as_bytes() == b"/*" {
        loop {
            match lex.read() {
                0 => return lex.token = Token::UnexpectedToken,
                b'*' => {
                    if lex.next() == b'/' {
                        lex.bump();
                        break;
                    }
                }
                _ => lex.bump(),
            }
        }
    }

    lex.advance();
}

#[cfg(test)]
mod test {
    use super::Token;
    use logos::Logos;

    // https://github.com/paritytech/lunarity/blob/master/lexer/src/token.rs
    fn assert_lex<T>(source: &str, tokens: T)
    where
        T: AsRef<[(Token, &'static str)]>,
    {
        let mut lex = Token::lexer(source);

        for &(ref token, slice) in tokens.as_ref() {
            assert!(
                lex.token == *token && lex.slice() == slice,
                "\n\n\n\tExpected {:?}({:?}), found {:?}({:?}) instead!\n\n\n",
                token,
                slice,
                lex.token,
                lex.slice()
            );
            lex.advance();
        }

        assert_eq!(lex.token, Token::End);
    }

    #[test]
    fn empty() {
        assert_lex("       ", []);
    }

    #[test]
    fn comments() {
        assert_lex("/* hello world */       ", []);
        assert_lex("/* hello\r\nworld */", []);
        assert_lex(" /* hello world */ bar", [(Token::Ident, "bar")]);
        assert_lex("    /*hell* /world*/", []);
        assert_lex(
            "  /* hello world  ",
            [(Token::UnexpectedToken, "/* hello world  ")],
        );
        assert_lex("<!-- -->", [(Token::CDO, "<!--"), (Token::CDC, "-->")]);
    }

    #[test]
    fn strings() {
        assert_lex(
            "\"strings\" 'are' '' cool",
            [
                (Token::String, "\"strings\""),
                (Token::String, "'are'"),
                (Token::String, "''"),
                (Token::Ident, "cool"),
            ],
        );
    }

    #[test]
    fn numbers() {
        assert_lex("5.2 .4", [(Token::Number, "5.2"), (Token::Number, ".4")]);
        assert_lex(
            "-10.2 -.2 4e-22",
            [
                (Token::Number, "-10.2"),
                (Token::Number, "-.2"),
                (Token::Number, "4e-22"),
            ],
        );
        assert_lex("-5 +5.2", [(Token::Number, "-5"), (Token::Number, "+5.2")]);
    }
}
