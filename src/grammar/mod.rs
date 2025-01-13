use alpha034::GlobalStatement;
use chumsky::{error::Rich, span::SimpleSpan};

pub mod alpha034;

#[derive(Debug, PartialEq, Clone)]
pub enum Grammar {
    Alpha034(Option<Vec<Spanned<GlobalStatement>>>),
}

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span);
pub type SpannedSemanticToken = Spanned<usize>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token(pub String);

impl ToString for Token {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl FromIterator<Token> for String {
    fn from_iter<I: IntoIterator<Item = Token>>(iter: I) -> Self {
        iter.into_iter().map(|t| t.0).collect()
    }
}

#[macro_export]
macro_rules! T {
    [$text:expr] => {
        Token($text.to_string())
    };
}

pub struct ParserResponse<'a> {
    pub ast: Grammar,
    pub errors: Vec<Rich<'a, String>>,
    pub semantic_tokens: Vec<SpannedSemanticToken>,
}

pub trait LSPAnalysis: Sync + Send {
    fn tokenize(&self, input: &str) -> Vec<Spanned<Token>>;
    fn parse<'a>(&self, input: &'a Vec<Spanned<Token>>) -> ParserResponse<'a>;
}

#[derive(PartialEq)]
pub enum JumpDefinitionResult {
    InFile(Span),
    OpenFile(String),
    None,
}
