use std::fmt;

enum Node {
    Add { left: Box<Node>, right: Box<Node> },
    Multiply { left: Box<Node>, right: Box<Node> },
    Number { value: i64 },
}

/// Plainly show the operation or value.
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Node::*;
        match self {
            Add { left, right } => write!(f, "{} + {}", left, right),
            Multiply { left, right } => write!(f, "{} * {}", left, right),
            Number { value } => write!(f, "{}", value),
        }
    }
}

/// The debug repr of a Node encloses the `Display` in `"«"` and `"»"`.
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Node::*;
        write!(f, "«")?;
        match self {
            Add { left, right } => write!(f, "{} + {}", left, right),
            Multiply { left, right } => write!(f, "{} * {}", left, right),
            Number { value } => write!(f, "{}", value),
        }?;
        write!(f, "»")
    }
}

impl Node {
    pub fn add(left: Node, right: Node) -> Node {
        let left = Box::new(left);
        let right = Box::new(right);
        Node::Add { left, right }
    }
    pub fn mul(left: Node, right: Node) -> Node {
        let left = Box::new(left);
        let right = Box::new(right);
        Node::Multiply { left, right }
    }
    pub fn num(value: i64) -> Node {
        Node::Number { value }
    }
}

fn main() {
    let expr = Node::mul(Node::add(Node::num(3), Node::num(5)), Node::num(2));

    println!("{:?}", expr);
}
