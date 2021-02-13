use super::*;

pub(super) fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    expr_binding_power(p, 0)
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) -> Option<CompletedMarker> {
    let mut lhs = lhs(p)?;

    loop {
        let op = if p.at(TokenKind::Plus) {
            BinaryOp::Add
        } else if p.at(TokenKind::Minus) {
            BinaryOp::Sub
        } else if p.at(TokenKind::Star) {
            BinaryOp::Mul
        } else if p.at(TokenKind::Slash) {
            BinaryOp::Div
        } else {
            // We’re not at an operator; we don’t know what to do next, so we return and let the
            // caller decide.
            break;
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            break;
        }

        // Eat the operator’s token.
        p.bump();

        let m = lhs.precede(p);
        let parsed_rhs = expr_binding_power(p, right_binding_power).is_some();
        lhs = m.complete(p, SyntaxKind::InfixExpr);

        if !parsed_rhs {
            break;
        }
    }

    Some(lhs)
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    let cm = if p.at(TokenKind::Num) {
        literal(p)
    } else if p.at(TokenKind::Ident) {
        variable_ref(p)
    } else if p.at(TokenKind::Minus) {
        prefix_expr(p)
    } else if p.at(TokenKind::LParen) {
        paren_expr(p)
    } else {
        p.error();
        return None;
    };

    Some(cm)
}

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

enum UnaryOp {
    Neg,
}

impl UnaryOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}

fn literal(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Num));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::Literal)
}

fn variable_ref(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Ident));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::VariableRef)
}

fn prefix_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Minus));

    let m = p.start();

    let op = UnaryOp::Neg;
    let ((), right_binding_power) = op.binding_power();

    // Eat the operator’s token.
    p.bump();

    expr_binding_power(p, right_binding_power);

    m.complete(p, SyntaxKind::PrefixExpr)
}

fn paren_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::LParen));

    let m = p.start();
    p.bump();
    expr_binding_power(p, 0);
    p.expect(TokenKind::RParen);

    m.complete(p, SyntaxKind::ParenExpr)
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_num() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Literal@0..3
    Num@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_num_preceded_by_whitespace() {
        check(
            "   9876",
            expect![[r#"
Root@0..7
  Whitespace@0..3 "   "
  Literal@3..7
    Num@3..7 "9876""#]],
        );
    }

    #[test]
    fn parse_num_followed_by_whitespace() {
        check(
            "999   ",
            expect![[r#"
Root@0..6
  Literal@0..6
    Num@0..3 "999"
    Whitespace@3..6 "   ""#]],
        );
    }

    #[test]
    fn parse_num_surrounded_by_whitespace() {
        check(
            " 123     ",
            expect![[r#"
Root@0..9
  Whitespace@0..1 " "
  Literal@1..9
    Num@1..4 "123"
    Whitespace@4..9 "     ""#]],
        );
    }

    #[test]
    fn parse_variable_ref() {
        check(
            "counter",
            expect![[r#"
Root@0..7
  VariableRef@0..7
    Ident@0..7 "counter""#]],
        );
    }

    #[test]
    fn parse_simple_infix_expression() {
        check(
            "1+2",
            expect![[r#"
Root@0..3
  InfixExpr@0..3
    Literal@0..1
      Num@0..1 "1"
    Plus@1..2 "+"
    Literal@2..3
      Num@2..3 "2""#]],
        );
    }

    #[test]
    fn parse_left_associative_infix_expression() {
        check(
            "1+2+3+4",
            expect![[r#"
Root@0..7
  InfixExpr@0..7
    InfixExpr@0..5
      InfixExpr@0..3
        Literal@0..1
          Num@0..1 "1"
        Plus@1..2 "+"
        Literal@2..3
          Num@2..3 "2"
      Plus@3..4 "+"
      Literal@4..5
        Num@4..5 "3"
    Plus@5..6 "+"
    Literal@6..7
      Num@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
Root@0..7
  InfixExpr@0..7
    InfixExpr@0..5
      Literal@0..1
        Num@0..1 "1"
      Plus@1..2 "+"
      InfixExpr@2..5
        Literal@2..3
          Num@2..3 "2"
        Star@3..4 "*"
        Literal@4..5
          Num@4..5 "3"
    Minus@5..6 "-"
    Literal@6..7
      Num@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_whitespace() {
        check(
            " 1 +   2* 3 ",
            expect![[r#"
Root@0..12
  Whitespace@0..1 " "
  InfixExpr@1..12
    Literal@1..3
      Num@1..2 "1"
      Whitespace@2..3 " "
    Plus@3..4 "+"
    Whitespace@4..7 "   "
    InfixExpr@7..12
      Literal@7..8
        Num@7..8 "2"
      Star@8..9 "*"
      Whitespace@9..10 " "
      Literal@10..12
        Num@10..11 "3"
        Whitespace@11..12 " ""#]],
        );
    }

    #[test]
    fn parse_infix_expression_interspersed_with_comments() {
        check(
            "
1
  + 1 // Add one
  + 10 // Add ten",
            expect![[r##"
Root@0..37
  Whitespace@0..1 "\n"
  InfixExpr@1..37
    InfixExpr@1..22
      Literal@1..5
        Num@1..2 "1"
        Whitespace@2..5 "\n  "
      Plus@5..6 "+"
      Whitespace@6..7 " "
      Literal@7..22
        Num@7..8 "1"
        Whitespace@8..9 " "
        Comment@9..19 "// Add one"
        Whitespace@19..22 "\n  "
    Plus@22..23 "+"
    Whitespace@23..24 " "
    Literal@24..37
      Num@24..26 "10"
      Whitespace@26..27 " "
      Comment@27..37 "// Add ten""##]],
        );
    }

    #[test]
    fn do_not_parse_operator_if_gettting_rhs_failed() {
        check(
            "(1+",
            expect![[r#"
Root@0..3
  ParenExpr@0..3
    LParen@0..1 "("
    InfixExpr@1..3
      Literal@1..2
        Num@1..2 "1"
      Plus@2..3 "+"
error at 2..3: expected Num, identifier, ‘-’ or ‘(’
error at 2..3: expected ‘)’"#]],
        );
    }

    #[test]
    fn parse_negation() {
        check(
            "-10",
            expect![[r#"
Root@0..3
  PrefixExpr@0..3
    Minus@0..1 "-"
    Literal@1..3
      Num@1..3 "10""#]],
        );
    }

    #[test]
    fn negation_has_higher_binding_power_than_binary_operators() {
        check(
            "-20+20",
            expect![[r#"
Root@0..6
  InfixExpr@0..6
    PrefixExpr@0..3
      Minus@0..1 "-"
      Literal@1..3
        Num@1..3 "20"
    Plus@3..4 "+"
    Literal@4..6
      Num@4..6 "20""#]],
        );
    }

    #[test]
    fn parse_nested_parentheses() {
        check(
            "((((((10))))))",
            expect![[r#"
Root@0..14
  ParenExpr@0..14
    LParen@0..1 "("
    ParenExpr@1..13
      LParen@1..2 "("
      ParenExpr@2..12
        LParen@2..3 "("
        ParenExpr@3..11
          LParen@3..4 "("
          ParenExpr@4..10
            LParen@4..5 "("
            ParenExpr@5..9
              LParen@5..6 "("
              Literal@6..8
                Num@6..8 "10"
              RParen@8..9 ")"
            RParen@9..10 ")"
          RParen@10..11 ")"
        RParen@11..12 ")"
      RParen@12..13 ")"
    RParen@13..14 ")""#]],
        );
    }

    #[test]
    fn parentheses_affect_precedence() {
        check(
            "5*(2+1)",
            expect![[r#"
Root@0..7
  InfixExpr@0..7
    Literal@0..1
      Num@0..1 "5"
    Star@1..2 "*"
    ParenExpr@2..7
      LParen@2..3 "("
      InfixExpr@3..6
        Literal@3..4
          Num@3..4 "2"
        Plus@4..5 "+"
        Literal@5..6
          Num@5..6 "1"
      RParen@6..7 ")""#]],
        );
    }

    #[test]
    fn parse_unclosed_parentheses() {
        check(
            "(foo",
            expect![[r#"
Root@0..4
  ParenExpr@0..4
    LParen@0..1 "("
    VariableRef@1..4
      Ident@1..4 "foo"
error at 1..4: expected ‘+’, ‘-’, ‘*’, ‘/’ or ‘)’"#]],
        );
    }
}
