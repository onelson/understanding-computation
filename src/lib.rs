use std::collections::HashMap;
use std::fmt;

pub mod big_step;
pub mod small_step;

#[derive(Clone)]
pub enum Value {
    Number(i64),
    Boolean(bool),
}

pub trait Printable {
    fn inspect(&self) -> String {
        format!("«{}»", self.to_s())
    }
    fn to_s(&self) -> String;
}

impl Printable for Value {
    fn to_s(&self) -> String {
        match &self {
            Value::Number(value) => format!("{}", value),
            Value::Boolean(value) => format!("{}", value),
        }
    }
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
