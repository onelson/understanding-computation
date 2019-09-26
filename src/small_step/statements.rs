use crate::small_step::expressions::{Expr, Value};
use crate::small_step::{Environment, Printable};
use std::rc::Rc;

/// Boxed version of a `Statement` (so they can be passed around generically).
pub type Stmt = Rc<Box<dyn Statement>>;

pub struct DoNothing;

pub trait Statement: Printable {
    fn is_reducible(&self) -> bool;
    fn reduce(&self, environment: &Environment) -> (Stmt, Environment);
    fn does_nothing(&self) -> bool {
        false
    }
}

impl Statement for DoNothing {
    fn is_reducible(&self) -> bool {
        false
    }

    fn reduce(&self, _environment: &Environment) -> (Rc<Box<dyn Statement>>, Environment) {
        panic!("Cannot Reduce")
    }

    /// Used exclusively to know when statements are "done."
    fn does_nothing(&self) -> bool {
        true
    }
}

impl From<DoNothing> for Stmt {
    fn from(statement: DoNothing) -> Self {
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
    pub fn new<S: Into<String>, E: Into<Expr>>(name: S, expression: E) -> Self {
        Self(name.into(), expression.into())
    }
}

impl Statement for Assign {
    fn is_reducible(&self) -> bool {
        true
    }

    fn reduce(&self, environment: &Environment) -> (Rc<Box<dyn Statement>>, Environment) {
        if self.1.is_reducible() {
            (
                Assign(self.0.clone(), self.1.reduce(environment)).into(),
                environment.clone(),
            )
        } else {
            (
                DoNothing.into(),
                environment.update(&self.0, self.1.as_value().cloned().unwrap()),
            )
        }
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
    fn is_reducible(&self) -> bool {
        true
    }
    fn reduce(&self, environment: &Environment) -> (Rc<Box<dyn Statement>>, Environment) {
        if self.0.is_reducible() {
            let cond_reduced = self.0.reduce(environment);
            (
                If::new(cond_reduced, self.1.clone(), self.2.clone()).into(),
                environment.clone(),
            )
        } else {
            let stmt = match self.0.as_value() {
                Some(Value::Boolean(true)) => self.1.clone(),
                Some(Value::Boolean(false)) => self.2.clone(),
                _ => panic!("Condition must be boolean."),
            };
            (stmt, environment.clone())
        }
    }
}

impl From<If> for Stmt {
    fn from(statement: If) -> Self {
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
    fn is_reducible(&self) -> bool {
        true
    }
    fn reduce(&self, environment: &Environment) -> (Rc<Box<dyn Statement>>, Environment) {
        if self.0.does_nothing() {
            (self.1.clone(), environment.clone())
        } else {
            let (first_reduced, reduced_env) = self.0.reduce(environment);
            (Sequence(first_reduced, self.1.clone()).into(), reduced_env)
        }
    }
}

impl From<Sequence> for Stmt {
    fn from(statement: Sequence) -> Self {
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
    fn is_reducible(&self) -> bool {
        true
    }
    fn reduce(&self, environment: &Environment) -> (Rc<Box<dyn Statement>>, Environment) {
        (
            If::new(
                self.0.clone(),
                Sequence(self.1.clone(), While::from(&self).into()),
                DoNothing,
            )
            .into(),
            environment.clone(),
        )
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
