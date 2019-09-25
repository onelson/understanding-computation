use crate::small_step::{Environment, Printable};
use std::rc::Rc;

/// Boxed version of an `Expression` (so they can be passed around generically).
pub type Expr = Rc<Box<dyn Expression>>;

pub trait Expression: Printable {
    fn is_reducible(&self) -> bool;
    fn reduce(&self, environment: &Environment) -> Expr;
    fn as_value(&self) -> Option<&Value> {
        None
    }
}

#[derive(Clone)]
pub enum Value {
    Number(i64),
    Boolean(bool),
}

impl Expression for Value {
    fn is_reducible(&self) -> bool {
        false
    }
    fn reduce(&self, _environment: &Environment) -> Expr {
        panic!("Cannot reduce a Value.")
    }
    fn as_value(&self) -> Option<&Value> {
        Some(&self)
    }
}

impl Printable for Value {
    fn to_s(&self) -> String {
        match &self {
            Value::Number(value) => format!("{}", value),
            Value::Boolean(value) => format!("{}", value),
        }
    }
}

impl From<Value> for Expr {
    fn from(e: Value) -> Self {
        Rc::new(Box::new(e))
    }
}

pub struct Add(Expr, Expr);

impl Add {
    pub fn new<T: Into<Expr>>(left: T, right: T) -> Self {
        Self(left.into(), right.into())
    }
}

impl Expression for Add {
    fn is_reducible(&self) -> bool {
        true
    }

    fn reduce(&self, environment: &Environment) -> Expr {
        match (self.0.is_reducible(), self.1.is_reducible()) {
            (true, _) => Add::new(self.0.reduce(environment), self.1.clone()).into(),
            (_, true) => Add::new(self.0.clone(), self.1.reduce(environment)).into(),
            _ => match (self.0.as_value(), self.1.as_value()) {
                (Some(Value::Number(a)), Some(Value::Number(b))) => Value::Number(a + b).into(),
                _ => panic!("Unexpected values"),
            },
        }
    }
}

impl Printable for Add {
    fn to_s(&self) -> String {
        format!("{} + {}", self.0.to_s(), self.1.to_s())
    }
}

impl From<Add> for Expr {
    fn from(e: Add) -> Self {
        Rc::new(Box::new(e))
    }
}

pub struct Multiply(Expr, Expr);

impl Multiply {
    pub fn new<T: Into<Expr>>(left: T, right: T) -> Self {
        Self(left.into(), right.into())
    }
}

impl Expression for Multiply {
    fn is_reducible(&self) -> bool {
        true
    }

    fn reduce(&self, environment: &Environment) -> Expr {
        match (self.0.is_reducible(), self.1.is_reducible()) {
            (true, _) => Multiply::new(self.0.reduce(environment), self.1.clone()).into(),
            (_, true) => Multiply::new(self.0.clone(), self.1.reduce(environment)).into(),
            _ => match (self.0.as_value(), self.1.as_value()) {
                (Some(Value::Number(a)), Some(Value::Number(b))) => Value::Number(a * b).into(),
                _ => panic!("Unexpected values"),
            },
        }
    }
}

impl Printable for Multiply {
    fn to_s(&self) -> String {
        format!("{} * {}", self.0.to_s(), self.1.to_s())
    }
}

impl From<Multiply> for Expr {
    fn from(e: Multiply) -> Self {
        Rc::new(Box::new(e))
    }
}

pub struct LessThan(Expr, Expr);

impl LessThan {
    pub fn new<T: Into<Expr>>(left: T, right: T) -> Self {
        Self(left.into(), right.into())
    }
}

impl Expression for LessThan {
    fn is_reducible(&self) -> bool {
        true
    }

    fn reduce(&self, environment: &Environment) -> Expr {
        match (self.0.is_reducible(), self.1.is_reducible()) {
            (true, _) => LessThan(self.0.reduce(environment), self.1.clone()).into(),
            (_, true) => LessThan(self.0.clone(), self.1.reduce(environment)).into(),
            _ => match (self.0.as_value(), self.1.as_value()) {
                (Some(Value::Number(a)), Some(Value::Number(b))) => Value::Boolean(a < b).into(),
                _ => panic!("Unexpected values"),
            },
        }
    }
}

impl Printable for LessThan {
    fn to_s(&self) -> String {
        format!("{} < {}", self.0.to_s(), self.1.to_s())
    }
}

impl From<LessThan> for Expr {
    fn from(e: LessThan) -> Self {
        Rc::new(Box::new(e))
    }
}

pub struct Variable(String);

impl Variable {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self(name.into())
    }
}

impl Expression for Variable {
    fn is_reducible(&self) -> bool {
        true
    }

    fn reduce(&self, environment: &Environment) -> Expr {
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
    fn from(e: Variable) -> Self {
        Rc::new(Box::new(e))
    }
}
