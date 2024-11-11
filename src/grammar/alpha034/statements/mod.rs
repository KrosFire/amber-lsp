use chumsky::prelude::*;

use super::{expressions::parse_expr, AmberParser, Spanned, Statement};

pub mod block;
pub mod comment;
pub mod failed;
pub mod if_cond;
pub mod keywords;
pub mod loops;
pub mod modifiers;
pub mod shorthands;
pub mod var_init;
pub mod var_set;

pub fn statement_parser<'a>() -> impl AmberParser<'a, Spanned<Statement>> {
    recursive(|stmnt| {
        choice((
            var_init::var_init_parser(stmnt.clone()),
            var_set::var_set_parser(stmnt.clone()),
            block::block_parser_statement(stmnt.clone()),
            if_cond::if_chain_parser(stmnt.clone()),
            if_cond::if_cond_parser(stmnt.clone()),
            shorthands::shorthand_parser(stmnt.clone()),
            loops::inf_loop_parser(stmnt.clone()),
            loops::iter_loop_parser(stmnt.clone()),
            keywords::keywords_parser(stmnt.clone()),
            modifiers::modifier_parser(),
            comment::comment_parser(),
            parse_expr(stmnt).map_with(|expr, e| (Statement::Expression(Box::new(expr)), e.span())),
        ))
        .boxed()
    })
    .boxed()
}
