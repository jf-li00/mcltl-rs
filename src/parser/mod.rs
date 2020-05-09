mod lexer;
pub mod parser;

#[cfg(test)]
mod test_parser {
    use super::*;
    use crate::expression::LTLExpression;

    #[test]
    fn it_should_parse_ltl_binary_expression() {
        let input = "(p U (q U p))";
        let lexer = lexer::Lexer::new(input);
        let parse_result = parser::parse(lexer);

        let expected = LTLExpression::U(
            Box::new(LTLExpression::Literal("p".into())),
            Box::new(LTLExpression::U(
                Box::new(LTLExpression::Literal("q".into())),
                Box::new(LTLExpression::Literal("p".into())),
            )),
        );

        assert!(parse_result.is_ok());
        assert_eq!(expected, parse_result.unwrap().expr);
    }

    #[test]
    fn it_should_parse_ltl_unary_expression() {
        let input = "G (not p)";
        let lexer = lexer::Lexer::new(input);
        let parse_result = parser::parse(lexer);

        let expected = LTLExpression::G(Box::new(LTLExpression::Not(Box::new(
            LTLExpression::Literal("p".into()),
        ))));

        assert!(parse_result.is_ok());
        assert_eq!(expected, parse_result.unwrap().expr);
    }

    #[test]
    fn it_should_parse_conjunctive_normal_form() {
        let input = "(p /\\ (q and T))";
        let lexer = lexer::Lexer::new(input);
        let parse_result = parser::parse(lexer);

        let expected = LTLExpression::And(
            Box::new(LTLExpression::Literal("p".into())),
            Box::new(LTLExpression::And(
                Box::new(LTLExpression::Literal("q".into())),
                Box::new(LTLExpression::True),
            )),
        );

        assert!(parse_result.is_ok());
        assert_eq!(expected, parse_result.unwrap().expr);
    }

    #[test]
    fn it_should_parse_disjunction_normal_form() {
        let input = "(p \\/ (q or F))";
        let lexer = lexer::Lexer::new(input);
        let parse_result = parser::parse(lexer);

        let expected = LTLExpression::Or(
            Box::new(LTLExpression::Literal("p".into())),
            Box::new(LTLExpression::Or(
                Box::new(LTLExpression::Literal("q".into())),
                Box::new(LTLExpression::False),
            )),
        );

        assert!(parse_result.is_ok());
        assert_eq!(expected, parse_result.unwrap().expr);
    }

    #[test]
    fn it_should_not_parse_malformed_binary_expression() {
        let input = "(p U)";
        let lexer = lexer::Lexer::new(input);
        let parse_result = parser::parse(lexer);

        assert!(parse_result.is_err());
    }

    #[test]
    fn it_should_not_parse_malformed_unary_expression() {
        let input = "G(p q)";
        let lexer = lexer::Lexer::new(input);
        let parse_result = parser::parse(lexer);

        assert!(parse_result.is_err());
    }
}
