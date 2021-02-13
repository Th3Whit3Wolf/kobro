use logos::Logos;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
#[logos(subpattern DecDigit  = r"[0-9]")]
#[logos(subpattern DecDigit_ = r"[0-9_]")]
pub enum TokenKind {
    #[error]
    Error,

    #[regex(r"\s+")]
    Whitespace,

    #[token("fn")]
    FnKw,

    #[token("for")]
    ForKw,

    #[token("if")]
    IfKw,

    #[token("in")]
    InKw,

    #[token("let")]
    LetKw,

    #[token("match")]
    MatchKw,

    #[token("false")]
    FalseKw,

    #[token("true")]
    TrueKw,

    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,

    #[regex("[0-9]+")]
    Num,

    #[regex(r"(?&DecDigit)(?&DecDigit_)*\.(?&DecDigit)(?&DecDigit_)*")]
    Float,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("=")]
    Equals,

    #[token("or")]
    LogicalOr,

    #[token("and")]
    LogicalAnd,

    #[token("not")]
    LogicalNot,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("<=")] // NOTE these aren't incorporated into the expr module yet
    LessThanEq,

    #[token(">=")]
    GreaterThanEq,

    #[token("<")]
    LessThan,

    #[token(">")]
    GreaterThan,

    #[token("!=")]
    NotEqual,

    #[token("==")]
    Equal,

    #[regex("//.*")]
    Comment,

    #[token(":")]
    Colon,

    #[token("=>")]
    FatArrow,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Whitespace => "whitespace",
            Self::FnKw => "‘fn’",
            Self::ForKw => "for",
            Self::IfKw => "if",
            Self::InKw => "in",
            Self::MatchKw => "match",
            Self::FalseKw => "‘false’",
            Self::TrueKw => "‘true’",
            Self::LetKw => "‘let’",
            Self::Ident => "identifier",
            Self::Num => "Num",
            Self::Float => "Float",
            Self::Plus => "‘+’",
            Self::Minus => "‘-’",
            Self::Star => "‘*’",
            Self::Slash => "‘/’",
            Self::Equals => "‘=’",
            Self::LParen => "‘(’",
            Self::RParen => "‘)’",
            Self::LBrace => "‘{’",
            Self::RBrace => "‘}’",
            Self::LBracket => "‘[’",
            Self::RBracket => "‘]’",
            Self::LessThanEq => ("<="),
            Self::GreaterThanEq => (">="),
            Self::LessThan => ("<"),
            Self::GreaterThan => (">"),
            Self::NotEqual => ("!="),
            Self::Equal => "`==`",
            Self::LogicalOr => "or",
            Self::LogicalAnd => "and",
            Self::LogicalNot => "not",
            Self::Comment => "comment",
            Self::Colon => "‘:’",
            Self::FatArrow => "‘=>’",
            Self::Error => "an unrecognized token",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lexer;

    fn check(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);

        let token = lexer.next().unwrap();
        assert_eq!(token.kind, kind);
        assert_eq!(token.text, input);
    }

    #[test]
    fn lex_spaces_and_newlines() {
        check("  \n ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_fn_keyword() {
        check("fn", TokenKind::FnKw);
    }

    #[test]
    fn lex_let_keyword() {
        check("let", TokenKind::LetKw);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", TokenKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("ab123cde456", TokenKind::Ident);
    }

    #[test]
    fn lex_mixed_case_identifier() {
        check("ABCdef", TokenKind::Ident);
    }

    #[test]
    fn lex_single_char_identifier() {
        check("x", TokenKind::Ident);
    }

    #[test]
    fn lex_num() {
        check("123456", TokenKind::Num);
    }

    #[test]
    fn lex_float() {
        check("30.45", TokenKind::Float);
    }

    #[test]
    fn lex_plus() {
        check("+", TokenKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", TokenKind::Minus);
    }

    #[test]
    fn lex_star() {
        check("*", TokenKind::Star);
    }

    #[test]
    fn lex_slash() {
        check("/", TokenKind::Slash);
    }

    #[test]
    fn lex_equals() {
        check("=", TokenKind::Equals);
    }

    #[test]
    fn lex_left_parenthesis() {
        check("(", TokenKind::LParen);
    }

    #[test]
    fn lex_right_parenthesis() {
        check(")", TokenKind::RParen);
    }

    #[test]
    fn lex_left_brace() {
        check("{", TokenKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check("}", TokenKind::RBrace);
    }

    #[test]
    fn lex_left_bracket() {
        check("[", TokenKind::LBracket);
    }

    #[test]
    fn lex_right_bracket() {
        check("]", TokenKind::RBracket);
    }

    #[test]
    fn lex_comment() {
        check("// foo", TokenKind::Comment);
    }
}
