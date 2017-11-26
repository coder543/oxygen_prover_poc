use std::fmt;

pub type Id = String;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Range {
    pub start: Option<i64>,
    pub end: Option<i64>,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(start) = self.start {
            write!(f, "{}", start)?;
        }

        write!(f, "..")?;

        if let Some(end) = self.end {
            write!(f, "{}", end)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constraint {
    pub name: Option<String>,
    pub ranges: Vec<Range>,
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref name) = self.name {
            if self.ranges.len() == 1 {
                write!(f, "{:?} ({})", name, self.ranges[0])?;
            } else if self.ranges.len() == 0 {
                write!(f, "{:?} (..)", name)?;
            } else {
                write!(f, "{:?} [", name)?;
                for range in &self.ranges {
                    write!(f, "{}, ", range)?;
                }
                write!(f, "\x08\x08]")?;
            }
        } else {
            if self.ranges.len() == 1 {
                write!(f, "({})", self.ranges[0])?;
            } else if self.ranges.len() == 0 {
                write!(f, "(..)")?;
            } else {
                write!(f, "[")?;
                for range in &self.ranges {
                    write!(f, "{}, ", range)?;
                }
                write!(f, "\x08\x08]")?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<(String, Constraint)>,
    pub result: Constraint,
    pub body: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Variable(Id),
    Value(Constraint),
    Assign(Id, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Call(Id, Vec<Expr>),
    Assert(Id, Range),
}

// q = quick

pub fn qval(v: i64) -> Expr {
    Expr::Value(Constraint {
        name: None,
        ranges: vec![
            Range {
                start: v.into(),
                end: v.into(),
            },
        ],
    })
}

pub fn qvar(name: &'static str) -> Expr {
    Expr::Variable(name.into())
}

pub fn qrange<IntoValA: Into<Option<i64>>, IntoValB: Into<Option<i64>>>(
    start: IntoValA,
    end: IntoValB,
) -> Range {
    let (start, end) = (start.into(), end.into());
    Range { start, end }
}

pub fn qconstraint<IntoName: Into<Option<&'static str>>, IntoVal: Into<Option<i64>>>(
    name: IntoName,
    start: IntoVal,
    end: IntoVal,
) -> Constraint {
    let name = name.into().map(|v| v.to_string());
    Constraint {
        name,
        ranges: vec![qrange(start, end)],
    }
}

pub fn unconstrained() -> Constraint {
    let name = "unconstrained".to_string().into();
    let ranges = vec![];
    Constraint { name, ranges }
}

pub fn qfn<IntoString: Into<String>>(
    name: IntoString,
    args: Vec<(String, Constraint)>,
    result: Constraint,
    body: Vec<Expr>,
) -> Function {
    let name = name.into();
    Function {
        name,
        args,
        result,
        body,
    }
}

pub fn qdiv(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Div(Box::new(lhs), Box::new(rhs))
}

pub fn qadd(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Add(Box::new(lhs), Box::new(rhs))
}

pub fn qassign(id: &'static str, rhs: Expr) -> Expr {
    let id = id.into();
    Expr::Assign(id, Box::new(rhs))
}

pub fn qcall(func: &'static str, args: Vec<Expr>) -> Expr {
    let func = func.into();
    Expr::Call(func, args)
}

pub fn qassert(id: &'static str, range: Range) -> Expr {
    let id = id.into();
    Expr::Assert(id, range)
}
