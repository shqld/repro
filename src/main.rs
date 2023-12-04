use swc_ecma_ast::{Pat, PatOrExpr};
use swc_ecma_parser::{Parser, StringInput, Syntax};

fn main() -> Result<(), &'static str> {
    let code = "
        o.n = 42
    ";

    let syntax = Syntax::Es(Default::default());
    let input = StringInput::new(code, Default::default(), Default::default());

    let mut parser = Parser::new(syntax, input, None);
    let expr = parser.parse_expr().unwrap();

    let assign = expr.expect_assign();

    match assign.left {
        PatOrExpr::Pat(pat) => match *pat {
            Pat::Expr(_) => {
                // According to ducumentation, it looks like this should not happen but happens.
                // https://github.com/swc-project/swc/blob/7327f257d2133ba2507990a73a11a2f58dfd826c/crates/swc_ecma_ast/src/pat.rs#L34-L36
                Err("PatOrExpr::Pat(Pat::Expr(_)) is not expected")
            }
            _ => unreachable!(),
        },
        PatOrExpr::Expr(_) => {
            //
            Ok(())
        }
    }
}
