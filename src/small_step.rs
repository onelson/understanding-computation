mod expressions;
mod statements;
use crate::{Environment, Printable};
pub use expressions::*;
pub use statements::*;

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
