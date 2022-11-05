use logos::{Logos};
use crate::position::*;
use crate::error::*;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    None,
    #[error]
    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    #[regex(r"//.+\n", logos::skip)]
    Error,

    #[token("+-")]
    PlusMinus,
    #[token("++")]
    Concat,
    #[token("+")]
    Plus,
    #[token("--")]
    Remove,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("^")]
    Exponent,
    #[token("%")]
    Percent,
    #[token("=")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("->")]
    ArrowRight,
    #[token("<-")]
    ArrowLeft,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("?")]
    Option,
    #[token(".")]
    Field,
    #[token("..")]
    Range,

    #[token("(")]
    EvalIn,
    #[token(")")]
    EvalOut,
    #[token("[")]
    VectorIn,
    #[token("]")]
    VectorOut,
    #[token("{")]
    SetIn,
    #[token("}")]
    SetOut,
    #[token("|")]
    Abs,
    #[token(",")]
    Sep,
    #[token(";")]
    End,
    #[token(":=")]
    Assign,
    #[token("::")]
    Def,
    #[token(":")]
    Rep,
    #[token("#")]
    Amount,

    #[regex(r"\d+", |lex| lex.slice().parse())]
    Int(i64),
    #[regex(r"\d+\.\d+", |lex| lex.slice().parse())]
    Float(f64),
    #[regex(r"[a-zA-Z_]|[a-zA-Z_][a-zA-Z_0-9]+", |lex| lex.slice().parse())]
    Word(String),
}
impl Token {
    pub fn name(&self) -> String {
        match self {
            Self::None => format!("end"),
            Self::Error => format!("error"),
            _ => format!("'{self}'")
        }
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "<none>"),
            Self::Error => write!(f, "<error>"),
            Self::PlusMinus => write!(f, "+-"),
            Self::Concat => write!(f, "++"),
            Self::Plus => write!(f, "+"),
            Self::Remove => write!(f, "--"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Exponent => write!(f, "^"),
            Self::Percent => write!(f, "%"),
            Self::Equal => write!(f, "="),
            Self::NotEqual => write!(f, "!="),
            Self::ArrowLeft => write!(f, "<-"),
            Self::ArrowRight => write!(f, "->"),
            Self::Less => write!(f, "<"),
            Self::Greater => write!(f, ">"),
            Self::LessEqual => write!(f, "<="),
            Self::GreaterEqual => write!(f, ">="),
            Self::Option => write!(f, "?"),
            Self::Field => write!(f, "."),
            Self::Range => write!(f, ".."),
            Self::EvalIn => write!(f, "("),
            Self::EvalOut => write!(f, ")"),
            Self::VectorIn => write!(f, "["),
            Self::VectorOut => write!(f, "]"),
            Self::SetIn => write!(f, "{{"),
            Self::SetOut => write!(f, "}}"),
            Self::Abs => write!(f, "|"),
            Self::Sep => write!(f, ","),
            Self::End => write!(f, ";"),
            Self::Assign => write!(f, ":="),
            Self::Def => write!(f, "::"),
            Self::Rep => write!(f, ":"),
            Self::Amount => write!(f, "#"),
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
            Self::Word(v) => write!(f, "{v}"),
        }
    }
}

pub fn lex(text: &str, path: &str) -> Result<(Vec<Token>, Vec<Position>), Error> {
    let mut lexer = Token::lexer(text);
    let mut tokens: Vec<Token> = vec![];
    let mut poses: Vec<Position> = vec![];
    loop {
        let res = lexer.next();
        match res {
            Some(token) => {
                if token == Token::Error {
                    return Err(Error::Char(lexer.slice().to_string()))
                }
                tokens.push(token);
                poses.push(Position(path.to_string(), lexer.span()));
            }
            None => break,
        }
    }
    Ok((tokens, poses))
}