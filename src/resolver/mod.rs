use ast::*;
use std::collections::HashMap;

pub fn resolve_fn(
    id: &str,
    args: &[Expr],
    scope: &HashMap<String, Constraint>,
    functions: &[Function],
) -> Constraint {
    for func in functions {
        if func.name == id {
            if args.len() != func.args.len() {
                panic!("wrong number of arguments for function {}", id);
            }

            for (arg, &(_, ref constraint)) in args.iter().zip(func.args.iter()) {
                let arg_constraint = resolve_expr(arg, scope, functions);
                if arg_constraint != *constraint {
                    'outer: for l_range in &constraint.ranges {
                        for r_range in &arg_constraint.ranges {
                            match (l_range.start, r_range.start) {
                                (Some(l_start), Some(r_start)) if l_start <= r_start => {}
                                (None, Some(_)) => {}
                                _ => {
                                    break;
                                }
                            }
                            match (l_range.end, r_range.end) {
                                (Some(l_end), Some(r_end)) if l_end >= r_end => continue 'outer,
                                (None, Some(_)) => continue 'outer,
                                _ => {}
                            }
                        }
                        panic!(
                            "{:#?} resolves to {} which is not a member of {}",
                            arg,
                            arg_constraint,
                            constraint
                        );
                    }
                }
            }

            return func.result.clone();
        }
    }

    panic!("function {:?} has not been defined", id);
}

pub fn resolve_expr(
    expr: &Expr,
    scope: &HashMap<String, Constraint>,
    functions: &[Function],
) -> Constraint {
    match *expr {
        Expr::Call(ref id, ref args) => resolve_fn(&id, &args, scope, functions),
        Expr::Assign(_, _) => panic!("nested assignment not allowed!"),
        Expr::Value(ref constraint) => constraint.clone(),
        Expr::Variable(ref id) => {
            if let Some(constraint) = scope.get(id) {
                constraint.clone()
            } else {
                panic!("variable {:?} has not been defined yet", id);
            }
        }
        Expr::Add(ref lhs, ref rhs) => {
            let lhs = resolve_expr(&lhs, scope, functions);
            let rhs = resolve_expr(&rhs, scope, functions);
            let mut ranges = vec![];

            for l_range in &lhs.ranges {
                for r_range in &rhs.ranges {
                    let start = l_range
                        .start
                        .map(|ls| r_range.start.map(|rs| rs + ls))
                        .unwrap_or(None);

                    let end = l_range
                        .end
                        .map(|ls| r_range.end.map(|rs| rs + ls))
                        .unwrap_or(None);

                    ranges.push(qrange(start, end));
                }
            }

            Constraint { name: None, ranges }
        }
        _ => unconstrained(),
    }
}

// this algorithm is wrong in cases where assert_range has a None
// and it's just not great.
pub fn resolve_assert(
    id: &String,
    assert_range: Range,
    scope: &HashMap<String, Constraint>,
) -> Constraint {
    let constraint = scope.get(id).expect("could not find variable").clone();
    let mut new_constraint = Constraint {
        name: None,
        ranges: vec![],
    };
    println!("assert_range: {}", assert_range);
    for range in constraint.ranges {
        println!("range: {}", range);
        let mut start = range.start;
        if start.is_none() {
            start = assert_range.start;
        }
        if start > assert_range.end {
            println!("start > assert_range.end");
            continue;
        }
        if start < assert_range.start {
            println!("start < assert_range.start");
            start = assert_range.start;
        }
        let mut end = range.end;
        if end.is_none() {
            end = assert_range.end;
        }
        if end < assert_range.start {
            println!("end < assert_range.start");
            continue;
        }
        if end > assert_range.end {
            println!("end > assert_range.end");
            end = assert_range.end;
        }
        new_constraint.ranges.push(qrange(start, end));
    }
    if new_constraint.ranges.len() == 0 {
        new_constraint.ranges.push(assert_range);
    }
    println!("new constraint: {}", new_constraint);
    new_constraint
}

pub fn resolve_exprs(exprs: Vec<Expr>, functions: Vec<Function>) {
    println!();
    println!("functions: {:#?}", functions);

    let mut scope = HashMap::new();
    for expr in exprs {
        println!("expr: {:#?}", expr);
        match expr {
            Expr::Assign(id, expr) => {
                let constraint = resolve_expr(&expr, &scope, &functions);
                scope.insert(id, constraint);
            }
            Expr::Assert(id, assert_range) => {
                let new_constraint = resolve_assert(&id, assert_range, &scope);
                scope.insert(id, new_constraint);
            }
            _ => {
                resolve_expr(&expr, &scope, &functions);
            }
        }
        println!("scope: {:#?}", scope);
        println!();
    }
}
