use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
    Whitespace,
    FnKw,
    ForKw,
    IfKw,
    InKw,
    TrueKw,
    FalseKw,
    MatchKw,
    LetKw,
    Ident,
    Num,
    Float,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LessThanEq,
    GreaterThanEq,
    LessThan,
    GreaterThan,
    NotEqual,
    Equal,
    LogicalOr,
    LogicalAnd,
    LogicalNot,
    Comment,
    Colon,
    FatArrow,
    Error,
    Root,
    InfixExpr,
    Literal,
    ParenExpr,
    PrefixExpr,
    VariableDef,
    VariableRef,
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::FnKw => Self::FnKw,
            TokenKind::ForKw => Self::ForKw,
            TokenKind::IfKw => Self::IfKw,
            TokenKind::InKw => Self::InKw,
            TokenKind::TrueKw => Self::TrueKw,
            TokenKind::FalseKw => Self::FalseKw,
            TokenKind::MatchKw => Self::MatchKw,
            TokenKind::LetKw => Self::LetKw,
            TokenKind::Ident => Self::Ident,
            TokenKind::Num => Self::Num,
            TokenKind::Float => Self::Float,
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Star => Self::Star,
            TokenKind::Slash => Self::Slash,
            TokenKind::Equals => Self::Equals,
            TokenKind::LParen => Self::LParen,
            TokenKind::RParen => Self::RParen,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::RBrace => Self::RBrace,
            TokenKind::LBracket => Self::LBracket,
            TokenKind::RBracket => Self::RBracket,
            TokenKind::LessThanEq => Self::LessThanEq,
            TokenKind::GreaterThanEq => Self::GreaterThanEq,
            TokenKind::LessThan => Self::LessThan,
            TokenKind::GreaterThan => Self::GreaterThan,
            TokenKind::NotEqual => Self::NotEqual,
            TokenKind::Equal => Self::Equal,
            TokenKind::LogicalOr => Self::LogicalOr,
            TokenKind::LogicalAnd => Self::LogicalAnd,
            TokenKind::LogicalNot => Self::LogicalNot,
            TokenKind::Comment => Self::Comment,
            TokenKind::Colon => Self::Colon,
            TokenKind::FatArrow => Self::FatArrow,
            TokenKind::Error => Self::Error,
        }
    }
}

pub type SyntaxNode = rowan::SyntaxNode<KobroLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<KobroLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<KobroLanguage>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum KobroLanguage {}

impl rowan::Language for KobroLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}
