use crate::big_step::expressions::Expr;
use crate::{Environment, Printable, Value};
use std::rc::Rc;

/// Boxed version of a `Statement` (so they can be passed around generically).
pub type Stmt = Rc<Box<dyn Statement>>;

pub struct DoNothing;

pub trait Statement: Printable {
    fn evaluate(&self, environment: &Environment) -> Environment;
}

impl Statement for DoNothing {
    fn evaluate(&self, environment: &Environment) -> Environment {
        environment.clone()
    }
}

impl From<DoNothing> for Stmt {
    fn from(statement: DoNothing) -> Stmt {
        Rc::new(Box::new(statement))
    }
}

impl Printable for DoNothing {
    fn to_s(&self) -> String {
        String::from("do-nothing")
    }
}

pub struct Assign(String, Expr);

impl Assign {
    pub fn new<N: Into<String>, E: Into<Expr>>(name: N, expression: E) -> Self {
        Self(name.into(), expression.into())
    }
}

impl Statement for Assign {
    fn evaluate(&self, environment: &Environment) -> Environment {
        environment.update(
            &self.0,
            self.1.evaluate(environment).as_value().unwrap().clone(),
        )
    }
}

impl From<Assign> for Stmt {
    fn from(statement: Assign) -> Self {
        Rc::new(Box::new(statement))
    }
}

impl Printable for Assign {
    fn to_s(&self) -> String {
        format!("{} = {}", &self.0, self.1.to_s())
    }
}

pub struct If(Expr, Stmt, Stmt);

impl If {
    pub fn new<E: Into<Expr>, S1: Into<Stmt>, S2: Into<Stmt>>(
        condition: E,
        consequence: S1,
        alternative: S2,
    ) -> Self {
        Self(condition.into(), consequence.into(), alternative.into())
    }
}

impl Statement for If {
    fn evaluate(&self, environment: &Environment) -> Environment {
        match self.0.evaluate(environment).as_value() {
            Some(Value::Boolean(true)) => self.1.evaluate(environment),
            Some(Value::Boolean(false)) => self.2.evaluate(environment),
            _ => panic!("Condition must be boolean."),
        }
    }
}

impl From<If> for Stmt {
    fn from(statement: If) -> Stmt {
        Rc::new(Box::new(statement))
    }
}

impl Printable for If {
    fn to_s(&self) -> String {
        format!(
            "if ({}) {{ {} }} else {{ {} }}",
            self.0.to_s(),
            self.1.to_s(),
            self.2.to_s()
        )
    }
}

pub struct Sequence(Stmt, Stmt);

impl Sequence {
    pub fn new<S1: Into<Stmt>, S2: Into<Stmt>>(first: S1, second: S2) -> Self {
        Self(first.into(), second.into())
    }
}

impl Statement for Sequence {
    fn evaluate(&self, environment: &Environment) -> Environment {
        self.1.evaluate(&self.0.evaluate(environment))
    }
}

impl From<Sequence> for Stmt {
    fn from(statement: Sequence) -> Stmt {
        Rc::new(Box::new(statement))
    }
}

impl Printable for Sequence {
    fn to_s(&self) -> String {
        format!("{}; {}", self.0.to_s(), self.1.to_s())
    }
}

pub struct While(Expr, Stmt);

impl While {
    pub fn new<E: Into<Expr>, S: Into<Stmt>>(condition: E, body: S) -> Self {
        Self(condition.into(), body.into())
    }

    /// Sort of like a `Clone`
    pub fn from(other: &While) -> Self {
        Self::new(other.0.clone(), other.1.clone())
    }
}

impl Statement for While {
    fn evaluate(&self, environment: &Environment) -> Environment {
        match self.0.evaluate(environment).as_value() {
            Some(Value::Boolean(true)) => self.evaluate(&self.1.evaluate(environment)),
            Some(Value::Boolean(false)) => environment.clone(),
            _ => unreachable!(),
        }
    }
}

impl From<While> for Stmt {
    fn from(statement: While) -> Self {
        Rc::new(Box::new(statement))
    }
}

impl Printable for While {
    fn to_s(&self) -> String {
        format!("while ({}) {{ {} }}", self.0.to_s(), self.1.to_s())
    }
}
