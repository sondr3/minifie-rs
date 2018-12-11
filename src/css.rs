use ::logos::{Lexer, Logos, Slice, Source};

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
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

    // https://www.w3.org/TR/css3-values/
    #[regex = "em|ex|ch|rem|vw|vh|vmin|vmax"]
    RelativeLength,
    #[regex = "cm|mm|Q|in|pc|pt|px"]
    AbsoluteLength,

    #[regex = "[+-]?[0-9]*[.]?[0-9]+(?:[eE][+-]?[0-9]+)?"]
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

    #[regex = "u\\+[0-9a-zA-Z]*"]
    #[regex = "U\\+[0-9a-zA-Z]*"]
    Unicode,
    #[token = "url"]
    URL,

    #[token = "("]
    ParenOpen,
    #[token = ")"]
    ParenClose,
    #[token = "{"]
    CurlyBracketOpen,
    #[token = "}"]
    CurlyBracketClose,
    #[token = "["]
    BracketOpen,
    #[token = "]"]
    BracketClose,

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
                        lex.bump(1);
                        break;
                    }
                }
                _ => lex.bump(1),
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
    fn assert_lex<'a, Source>(source: Source, tokens: &[(Token, Source::Slice)])
    where
        Source: logos::Source<'a>,
    {
        let mut lex = Token::lexer(source);

        for tuple in tokens {
            assert_eq!(&(lex.token, lex.slice()), tuple);

            lex.advance();
        }

        assert_eq!(lex.token, Token::End);
    }

    #[test]
    fn empty() {
        assert_lex("       ", &[]);
    }

    #[test]
    fn comments() {
        assert_lex("/* hello world */       ", &[]);
        assert_lex("/* hello\r\nworld */", &[]);
        assert_lex(" /* hello world */ bar", &[(Token::Ident, "bar")]);
        assert_lex("    /*hell* /world*/", &[]);
        assert_lex(
            "  /* hello world  ",
            &[(Token::UnexpectedToken, "/* hello world  ")],
        );
        assert_lex("<!-- -->", &[(Token::CDO, "<!--"), (Token::CDC, "-->")]);
    }

    #[test]
    fn strings() {
        assert_lex(
            "\"strings\" 'are' '' cool",
            &[
                (Token::String, "\"strings\""),
                (Token::String, "'are'"),
                (Token::String, "''"),
                (Token::Ident, "cool"),
            ],
        );
    }

    #[test]
    fn numbers() {
        assert_lex("5.2 .4", &[(Token::Number, "5.2"), (Token::Number, ".4")]);
        assert_lex(
            "-10.2 -.2 4e-22 53E-22 67e+3 5.2E+22 5 -5 2E+1 8E-0",
            &[
                (Token::Number, "-10.2"),
                (Token::Number, "-.2"),
                (Token::Number, "4e-22"),
                (Token::Number, "53E-22"),
                (Token::Number, "67e+3"),
                (Token::Number, "5.2E+22"),
                (Token::Number, "5"),
                (Token::Number, "-5"),
                (Token::Number, "2E+1"),
                (Token::Number, "8E-0"),
            ],
        );
        assert_lex("-5 +5.2", &[(Token::Number, "-5"), (Token::Number, "+5.2")]);
    }

    #[test]
    fn unicode() {
        assert_lex(
            "U+1234 u+1AAF U+A69F u+FaFF U+2FA1F U+1D7fF",
            &[
                (Token::Unicode, "U+1234"),
                (Token::Unicode, "u+1AAF"),
                (Token::Unicode, "U+A69F"),
                (Token::Unicode, "u+FaFF"),
                (Token::Unicode, "U+2FA1F"),
                (Token::Unicode, "U+1D7fF"),
            ],
        );
    }

    #[test]
    fn controls() {
        assert_lex(
            "; : , . ( ) [ ] { }",
            &[
                (Token::Semicolon, ";"),
                (Token::Colon, ":"),
                (Token::Comma, ","),
                (Token::Period, "."),
                (Token::ParenOpen, "("),
                (Token::ParenClose, ")"),
                (Token::BracketOpen, "["),
                (Token::BracketClose, "]"),
                (Token::CurlyBracketOpen, "{"),
                (Token::CurlyBracketClose, "}"),
            ],
        );
    }

    #[test]
    fn idents() {
        assert_lex(
            "color: red;",
            &[
                (Token::Ident, "color"),
                (Token::Colon, ":"),
                (Token::Ident, "red"),
                (Token::Semicolon, ";"),
            ],
        );
        assert_lex(
            ".class { }",
            &[
                (Token::Period, "."),
                (Token::Ident, "class"),
                (Token::CurlyBracketOpen, "{"),
                (Token::CurlyBracketClose, "}"),
            ],
        );
        assert_lex(
            "#class { }",
            &[
                (Token::Hash, "#"),
                (Token::Ident, "class"),
                (Token::CurlyBracketOpen, "{"),
                (Token::CurlyBracketClose, "}"),
            ],
        );
        assert_lex(
            "@media print { }",
            &[
                (Token::At, "@"),
                (Token::Ident, "media"),
                (Token::Ident, "print"),
                (Token::CurlyBracketOpen, "{"),
                (Token::CurlyBracketClose, "}"),
            ],
        );
        assert_lex(
            "body { font: \"ComicSans\" }",
            &[
                (Token::Ident, "body"),
                (Token::CurlyBracketOpen, "{"),
                (Token::Ident, "font"),
                (Token::Colon, ":"),
                (Token::String, "\"ComicSans\""),
                (Token::CurlyBracketClose, "}"),
            ],
        );
    }

    #[test]
    fn relative_lengths() {
        assert_lex(
            ".container { width: 40em; } /* em  */ ",
            &[
                (Token::Period, "."),
                (Token::Ident, "container"),
                (Token::CurlyBracketOpen, "{"),
                (Token::Ident, "width"),
                (Token::Colon, ":"),
                (Token::Number, "40"),
                (Token::RelativeLength, "em"),
                (Token::CurlyBracketClose, "}"),
            ],
        );
    }

    #[test]
    fn absolute_lengths() {
        assert_lex(
            "h1 { margin: 0.5in }      /* inches  */",
            &[
                (Token::Ident, "h1"),
                (Token::CurlyBracketOpen, "{"),
                (Token::Ident, "margin"),
                (Token::Colon, ":"),
                (Token::Number, "0.5"),
                (Token::AbsoluteLength, "in"),
                (Token::CurlyBracketClose, "}"),
            ],
        );
        assert_lex(
            "h2 { \
             line-height: 3cm }   \
             /* centimeters */",
            &[
                (Token::Ident, "h2"),
                (Token::CurlyBracketOpen, "{"),
                (Token::Ident, "line-height"),
                (Token::Colon, ":"),
                (Token::Number, "3"),
                (Token::AbsoluteLength, "cm"),
                (Token::CurlyBracketClose, "}"),
            ],
        );
        assert_lex(
            "h3 { word-spacing: 4mm }  /* millimeters */",
            &[
                (Token::Ident, "h3"),
                (Token::CurlyBracketOpen, "{"),
                (Token::Ident, "word-spacing"),
                (Token::Colon, ":"),
                (Token::Number, "4"),
                (Token::AbsoluteLength, "mm"),
                (Token::CurlyBracketClose, "}"),
            ],
        );
        assert_lex(
            "h3 { letter-spacing: 1Q } /* quarter-millimeters */",
            &[
                (Token::Ident, "h3"),
                (Token::CurlyBracketOpen, "{"),
                (Token::Ident, "letter-spacing"),
                (Token::Colon, ":"),
                (Token::Number, "1"),
                (Token::AbsoluteLength, "Q"),
                (Token::CurlyBracketClose, "}"),
            ],
        );
        assert_lex(
            "h4 { font-size: 12pt }    /* points */",
            &[
                (Token::Ident, "h4"),
                (Token::CurlyBracketOpen, "{"),
                (Token::Ident, "font-size"),
                (Token::Colon, ":"),
                (Token::Number, "12"),
                (Token::AbsoluteLength, "pt"),
                (Token::CurlyBracketClose, "}"),
            ],
        );
        assert_lex(
            "h4 { font-size: 1pc }     /* picas */",
            &[
                (Token::Ident, "h4"),
                (Token::CurlyBracketOpen, "{"),
                (Token::Ident, "font-size"),
                (Token::Colon, ":"),
                (Token::Number, "1"),
                (Token::AbsoluteLength, "pc"),
                (Token::CurlyBracketClose, "}"),
            ],
        );
        assert_lex(
            "p  { font-size: 12px }    /* px */",
            &[
                (Token::Ident, "p"),
                (Token::CurlyBracketOpen, "{"),
                (Token::Ident, "font-size"),
                (Token::Colon, ":"),
                (Token::Number, "12"),
                (Token::AbsoluteLength, "px"),
                (Token::CurlyBracketClose, "}"),
            ],
        );
    }
}
