use std::rc::Rc;

trait Expression {
    fn is_reducible(&self) -> bool;
    fn reduce(&self) -> Option<Expr>;
    fn inspect(&self) -> String {
        format!("«{}»", self.to_s())
    }
    fn to_s(&self) -> String;
    fn as_number(&self) -> Option<&Number> {
        None
    }
}

type Expr = Rc<Box<dyn Expression>>;

#[derive(Clone)]
struct Number(i64);

impl Expression for Number {
    fn is_reducible(&self) -> bool {
        false
    }

    fn reduce(&self) -> Option<Expr> {
        None
    }

    fn to_s(&self) -> String {
        match &self {
            Number(value) => format!("{}", value),
        }
    }

    fn as_number(&self) -> Option<&Number> {
        Some(&self)
    }
}

impl From<Number> for Expr {
    fn from(number: Number) -> Self {
        Rc::new(Box::new(number))
    }
}

struct Add(Expr, Expr);

impl Expression for Add {
    fn is_reducible(&self) -> bool {
        true
    }

    fn reduce(&self) -> Option<Expr> {
        match (self.0.is_reducible(), self.1.is_reducible()) {
            (true, _) => Some(Add(self.0.reduce().unwrap(), self.1.clone()).into()),
            (_, true) => Some(Add(self.0.clone(), self.1.reduce().unwrap()).into()),
            _ => match (self.0.as_number(), self.1.as_number()) {
                (Some(Number(a)), Some(Number(b))) => Some(Number(a + b).into()),
                _ => panic!("Unexpected values"),
            },
        }
    }

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

    fn reduce(&self) -> Option<Expr> {
        match (self.0.is_reducible(), self.1.is_reducible()) {
            (true, _) => Some(Multiply(self.0.reduce().unwrap(), self.1.clone()).into()),
            (_, true) => Some(Multiply(self.0.clone(), self.1.reduce().unwrap()).into()),
            _ => match (self.0.as_number(), self.1.as_number()) {
                (Some(Number(a)), Some(Number(b))) => Some(Number(a * b).into()),
                _ => panic!("Unexpected values"),
            },
        }
    }
    fn to_s(&self) -> String {
        format!("{} * {}", self.0.to_s(), self.1.to_s())
    }
}

impl From<Multiply> for Expr {
    fn from(multiply: Multiply) -> Self {
        Rc::new(Box::new(multiply))
    }
}

fn main() {
    let mut expr: Expr = Multiply(
        Add(Number(3).into(), Number(5).into()).into(),
        Number(2).into(),
    )
    .into();

    println!("{}", &expr.inspect());

    while expr.is_reducible() {
        expr = expr.reduce().unwrap();
        println!("{}", expr.inspect());
    }
}
