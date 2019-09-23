use crate::small_step::expressions::Expr;
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
    fn from(do_nothing: DoNothing) -> Self {
        Rc::new(Box::new(do_nothing))
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
                Assign(self.0.clone(), self.1.reduce(environment).unwrap()).into(),
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
    fn from(assign: Assign) -> Self {
        Rc::new(Box::new(assign))
    }
}

impl Printable for Assign {
    fn to_s(&self) -> String {
        format!("{} = {}", &self.0, self.1.to_s())
    }
}

// TODO: If

pub struct Sequence(Stmt, Stmt);

impl Sequence {
    pub fn new<T: Into<Stmt>>(first: T, second: T) -> Self {
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
    fn from(sequence: Sequence) -> Self {
        Rc::new(Box::new(sequence))
    }
}

impl Printable for Sequence {
    fn to_s(&self) -> String {
        format!("{}; {}", self.0.to_s(), self.1.to_s())
    }
}

// TODO: While
