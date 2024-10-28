use crate::{
    grammar::alpha034::{
        lexer::Token, parser::ident, statements::failed::failure_parser, AmberParser, Expression,
        Spanned, Statement,
    },
    T,
};
use chumsky::prelude::*;

pub fn function_call_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
    expr: impl AmberParser<'a, Spanned<Expression>>,
) -> impl AmberParser<'a, Spanned<Expression>> {
    ident("function".to_string())
        .map_with(|name, e| (name, e.span()))
        .then_ignore(just(T!["("]))
        .then(
            expr.recover_with(via_parser(
                none_of([T![")"]]).map_with(|_, e| (Expression::Error, e.span())),
            ))
            .separated_by(
                just(T![","])
                    .recover_with(via_parser(none_of([T![")"]]).rewind().map(|_| T![","]))),
            )
            .collect(),
        )
        .then_ignore(just(T![")"]).recover_with(via_parser(any().or_not().map(|_| T![")"]))))
        .then(failure_parser(stmnts).or_not())
        .map_with(|((name, args), failure), e| {
            (
                Expression::FunctionInvocation(name, args, failure),
                e.span(),
            )
        })
}
