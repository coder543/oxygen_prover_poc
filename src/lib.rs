#![allow(unused)]

mod ast;
mod resolver;

use ast::*;
use resolver::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resolve() {
        let height_mm = qconstraint("HeightMm", 0, 3000);
        let height_in = qconstraint("HeightIn", 0, 182);

        let input = qfn("input", vec![], unconstrained(), vec![]);
        let to_inches = qfn(
            "toInches",
            vec![("z".into(), height_mm)],
            height_in,
            vec![qdiv(qvar("z"), qval(25))],
        );

        let functions = vec![input, to_inches];
        let exprs = vec![
            qassign("height", qcall("input", vec![])),
            qassert("height", qrange(0, 2997)),
            qassign("height", qadd(qvar("height"), qval(2))),
            qassign("height_in", qcall("toInches", vec![qvar("height")])),
        ];

        resolve_exprs(exprs, functions);

        panic!();
    }
}
