use crate::{Environment, Printable, Value};
use std::rc::Rc;

/// Boxed version of an `Expression` (so they can be passed around generically).
pub type Expr = Rc<Box<dyn Expression>>;

pub trait Expression: Printable {
    fn evaluate(&self, environment: &Environment) -> Expr;
    fn as_value(&self) -> Option<&Value> {
        None
    }
}

impl Expression for Value {
    fn evaluate(&self, _: &Environment) -> Expr {
        self.clone().into()
    }

    fn as_value(&self) -> Option<&Value> {
        Some(&self)
    }
}

impl From<Value> for Expr {
    fn from(expression: Value) -> Self {
        Rc::new(Box::new(expression))
    }
}

pub struct Add(Expr, Expr);

impl Add {
    pub fn new<T1: Into<Expr>, T2: Into<Expr>>(left: T1, right: T2) -> Self {
        Self(left.into(), right.into())
    }
}

impl Expression for Add {
    fn evaluate(&self, environment: &Environment) -> Expr {
        match (
            self.0.evaluate(environment).as_value(),
            self.1.evaluate(environment).as_value(),
        ) {
            (Some(Value::Number(a)), Some(Value::Number(b))) => Value::Number(a + b).into(),
            _ => panic!("Unexpected values"),
        }
    }
}

impl Printable for Add {
    fn to_s(&self) -> String {
        format!("{} + {}", self.0.to_s(), self.1.to_s())
    }
}

impl From<Add> for Expr {
    fn from(expression: Add) -> Self {
        Rc::new(Box::new(expression))
    }
}

pub struct Multiply(Expr, Expr);

impl Multiply {
    pub fn new<T1: Into<Expr>, T2: Into<Expr>>(left: T1, right: T2) -> Self {
        Self(left.into(), right.into())
    }
}

impl Expression for Multiply {
    fn evaluate(&self, environment: &Environment) -> Expr {
        match (
            self.0.evaluate(environment).as_value(),
            self.1.evaluate(environment).as_value(),
        ) {
            (Some(Value::Number(a)), Some(Value::Number(b))) => Value::Number(a * b).into(),
            _ => panic!("Unexpected values"),
        }
    }
}

impl Printable for Multiply {
    fn to_s(&self) -> String {
        format!("{} * {}", self.0.to_s(), self.1.to_s())
    }
}

impl From<Multiply> for Expr {
    fn from(expression: Multiply) -> Self {
        Rc::new(Box::new(expression))
    }
}

pub struct LessThan(Expr, Expr);

impl LessThan {
    pub fn new<T1: Into<Expr>, T2: Into<Expr>>(left: T1, right: T2) -> Self {
        Self(left.into(), right.into())
    }
}

impl Expression for LessThan {
    fn evaluate(&self, environment: &Environment) -> Expr {
        match (
            self.0.evaluate(environment).as_value(),
            self.1.evaluate(environment).as_value(),
        ) {
            (Some(Value::Number(a)), Some(Value::Number(b))) => Value::Boolean(a < b).into(),
            _ => panic!("Unexpected values"),
        }
    }
}

impl Printable for LessThan {
    fn to_s(&self) -> String {
        format!("{} < {}", self.0.to_s(), self.1.to_s())
    }
}

impl From<LessThan> for Expr {
    fn from(expression: LessThan) -> Self {
        Rc::new(Box::new(expression))
    }
}

pub struct Variable(String);

impl Variable {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self(name.into())
    }
}

impl Expression for Variable {
    fn evaluate(&self, environment: &Environment) -> Expr {
        let value = &environment.0[&self.0];
        value.clone().into()
    }
}

impl Printable for Variable {
    fn to_s(&self) -> String {
        self.0.clone()
    }
}

impl From<Variable> for Expr {
    fn from(expression: Variable) -> Self {
        Rc::new(Box::new(expression))
    }
}
