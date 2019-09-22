use std::rc::Rc;

trait Expression {
    fn is_reducible(&self) -> bool;
    fn reduce(&self) -> Option<Expr>;
    fn inspect(&self) -> String {
        format!("«{}»", self.to_s())
    }
    fn to_s(&self) -> String;
    fn as_value(&self) -> Option<&Value> {
        None
    }
}

type Expr = Rc<Box<dyn Expression>>;

#[derive(Clone)]
enum Value {
    Number(i64),
}

impl Expression for Value {
    fn is_reducible(&self) -> bool {
        false
    }

    fn reduce(&self) -> Option<Expr> {
        None
    }

    fn to_s(&self) -> String {
        match &self {
            Value::Number(value) => format!("{}", value),
        }
    }

    fn as_value(&self) -> Option<&Value> {
        Some(&self)
    }
}

struct Add(Expr, Expr);

impl Expression for Add {
    fn is_reducible(&self) -> bool {
        true
    }

    fn reduce(&self) -> Option<Expr> {
        match (self.0.is_reducible(), self.1.is_reducible()) {
            (true, _) => Some(Rc::new(Box::new(Add(
                self.0.reduce().unwrap(),
                self.1.clone(),
            )))),
            (_, true) => Some(Rc::new(Box::new(Add(
                self.0.clone(),
                self.1.reduce().unwrap(),
            )))),
            _ => match (self.0.as_value(), self.1.as_value()) {
                (Some(Value::Number(a)), Some(Value::Number(b))) => {
                    Some(Rc::new(Box::new(Value::Number(a + b))))
                }
                _ => panic!("Unexpected values"),
            },
        }
    }

    fn to_s(&self) -> String {
        format!("{} + {}", self.0.to_s(), self.1.to_s())
    }
}

struct Multiply(Expr, Expr);

impl Expression for Multiply {
    fn is_reducible(&self) -> bool {
        true
    }

    fn reduce(&self) -> Option<Expr> {
        match (self.0.is_reducible(), self.1.is_reducible()) {
            (true, _) => Some(Rc::new(Box::new(Multiply(
                self.0.reduce().unwrap(),
                self.1.clone(),
            )))),
            (_, true) => Some(Rc::new(Box::new(Multiply(
                self.0.clone(),
                self.1.reduce().unwrap(),
            )))),
            _ => match (self.0.as_value(), self.1.as_value()) {
                (Some(Value::Number(a)), Some(Value::Number(b))) => {
                    Some(Rc::new(Box::new(Value::Number(a * b))))
                }
                _ => panic!("Unexpected values"),
            },
        }
    }
    fn to_s(&self) -> String {
        format!("{} * {}", self.0.to_s(), self.1.to_s())
    }
}

fn main() {
    let mut expr: Expr = Rc::new(Box::new(Multiply(
        Rc::new(Box::new(Add(
            Rc::new(Box::new(Value::Number(3))),
            Rc::new(Box::new(Value::Number(5))),
        ))),
        Rc::new(Box::new(Value::Number(2))),
    )));

    println!("{}", &expr.inspect());

    while expr.is_reducible() {
        expr = expr.reduce().unwrap();
        println!("{}", expr.inspect());
    }
}
