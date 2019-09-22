use crate::Value::Number;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
struct Environment(HashMap<String, Value>);

impl Environment {
    pub fn merge(&self, key: &String, value: Value) -> Environment {
        let mut map = self.0.clone();
        map.insert(key.clone(), value);
        Environment(map)
    }
}
type Expr = Rc<Box<dyn Expression>>;
type Stmt = Rc<Box<dyn Statement>>;

trait Printable {
    fn inspect(&self) -> String {
        format!("«{}»", self.to_s())
    }
    fn to_s(&self) -> String;
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{{ ")?;
        for (key, value) in &self.0 {
            match value {
                Value::Number(val) => write!(f, "{}={}", key, val)?,
                Value::Boolean(val) => write!(f, "{}={}", key, val)?,
            }
        }
        write!(f, " }}")
    }
}

struct Machine {
    statement: Stmt,
    environment: Environment,
}

impl Machine {
    pub fn new<S: Into<Stmt>>(stmt: S) -> Self {
        Self {
            statement: stmt.into(),
            environment: Environment(HashMap::new()),
        }
    }
    fn print(&self) {
        println!("{}, {}", self.statement.inspect(), &self.environment);
    }

    fn step(&mut self) {
        let (statement, environment) = self.statement.reduce(&self.environment);
        self.statement = statement;
        self.environment = environment;
    }

    pub fn run(&mut self) {
        while self.statement.is_reducible() {
            self.print();
            self.step();
        }
        self.print();
    }
}

trait Expression: Printable {
    fn is_reducible(&self) -> bool;
    fn reduce(&self, environment: &Environment) -> Option<Expr>;
    fn as_value(&self) -> Option<&Value> {
        None
    }
}

trait Statement: Printable {
    fn is_reducible(&self) -> bool;
    fn reduce(&self, environment: &Environment) -> (Stmt, Environment);
}

#[derive(Clone)]
enum Value {
    Number(i64),
    Boolean(bool),
}

impl Expression for Value {
    fn is_reducible(&self) -> bool {
        false
    }
    fn reduce(&self, _environment: &Environment) -> Option<Expr> {
        None
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
    fn from(value: Value) -> Self {
        Rc::new(Box::new(value))
    }
}

struct Add(Expr, Expr);

impl Expression for Add {
    fn is_reducible(&self) -> bool {
        true
    }

    fn reduce(&self, environment: &Environment) -> Option<Expr> {
        match (self.0.is_reducible(), self.1.is_reducible()) {
            (true, _) => Some(Add(self.0.reduce(environment).unwrap(), self.1.clone()).into()),
            (_, true) => Some(Add(self.0.clone(), self.1.reduce(environment).unwrap()).into()),
            _ => match (self.0.as_value(), self.1.as_value()) {
                (Some(Value::Number(a)), Some(Value::Number(b))) => {
                    Some(Value::Number(a + b).into())
                }
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
    fn from(add: Add) -> Self {
        Rc::new(Box::new(add))
    }
}

struct Multiply(Expr, Expr);

impl Expression for Multiply {
    fn is_reducible(&self) -> bool {
        true
    }

    fn reduce(&self, environment: &Environment) -> Option<Expr> {
        match (self.0.is_reducible(), self.1.is_reducible()) {
            (true, _) => Some(Multiply(self.0.reduce(environment).unwrap(), self.1.clone()).into()),
            (_, true) => Some(Multiply(self.0.clone(), self.1.reduce(environment).unwrap()).into()),
            _ => match (self.0.as_value(), self.1.as_value()) {
                (Some(Value::Number(a)), Some(Value::Number(b))) => {
                    Some(Value::Number(a * b).into())
                }
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
    fn from(multiply: Multiply) -> Self {
        Rc::new(Box::new(multiply))
    }
}

struct LessThan(Expr, Expr);

impl Expression for LessThan {
    fn is_reducible(&self) -> bool {
        true
    }

    fn reduce(&self, environment: &Environment) -> Option<Expr> {
        match (self.0.is_reducible(), self.1.is_reducible()) {
            (true, _) => Some(LessThan(self.0.reduce(environment).unwrap(), self.1.clone()).into()),
            (_, true) => Some(LessThan(self.0.clone(), self.1.reduce(environment).unwrap()).into()),
            _ => match (self.0.as_value(), self.1.as_value()) {
                (Some(Value::Number(a)), Some(Value::Number(b))) => {
                    Some(Value::Boolean(a < b).into())
                }
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
    fn from(less_than: LessThan) -> Self {
        Rc::new(Box::new(less_than))
    }
}

struct Variable(String);

impl Printable for Variable {
    fn to_s(&self) -> String {
        self.0.clone()
    }
}

impl Expression for Variable {
    fn is_reducible(&self) -> bool {
        true
    }

    fn reduce(&self, environment: &Environment) -> Option<Rc<Box<dyn Expression>>> {
        let value = &environment.0[&self.0];
        Some(value.clone().into())
    }
}

struct DoNothing;

impl Statement for DoNothing {
    fn is_reducible(&self) -> bool {
        false
    }

    fn reduce(&self, _environment: &Environment) -> (Rc<Box<dyn Statement>>, Environment) {
        panic!("Cannot Reduce")
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

struct Assign(String, Expr);

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
                environment.merge(&self.0, self.1.as_value().cloned().unwrap()),
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

fn main() {
    let mut machine = Machine::new(Assign(
        "x".into(),
        Add(Number(3).into(), Number(5).into()).into(),
    ));
    machine.run();
}
