mod expressions;
mod statements;

pub use expressions::*;
pub use statements::*;
use std::collections::HashMap;
use std::fmt;

pub trait Printable {
    fn inspect(&self) -> String {
        format!("«{}»", self.to_s())
    }
    fn to_s(&self) -> String;
}

#[derive(Clone)]
pub struct Environment(HashMap<String, Value>);

impl Environment {
    /// Adds or replaces a key in the map, returning a new map.
    pub fn update(&self, key: &String, value: Value) -> Environment {
        let mut map = self.0.clone();
        map.insert(key.clone(), value);
        Environment(map)
    }

    pub fn empty() -> Self {
        Self(HashMap::new())
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{{ ")?;

        let locals: String = self
            .0
            .iter()
            .map(|(key, value)| match value {
                Value::Number(val) => format!("{}={}", key, val),
                Value::Boolean(val) => format!("{}={}", key, val),
            })
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", locals)?;
        write!(f, " }}")
    }
}

pub struct Machine {
    statement: Stmt,
    environment: Environment,
}

impl Machine {
    pub fn new<S: Into<Stmt>>(stmt: S) -> Self {
        Self {
            statement: stmt.into(),
            environment: Environment::empty(),
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
