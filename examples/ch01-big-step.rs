use uc::{big_step::*, Environment, Printable, Value::*};

fn main() {
    println!(
        "{}",
        LessThan::new(Number(5), Number(8))
            .evaluate(&Environment::empty())
            .inspect()
    );
    println!(
        "{}",
        LessThan::new(Number(2), Number(2))
            .evaluate(&Environment::empty())
            .inspect()
    );
    println!(
        "{}",
        LessThan::new(Number(18), Number(2))
            .evaluate(&Environment::empty())
            .inspect()
    );

    let seq = Sequence::new(
        Assign::new("x", Number(0)),
        While::new(
            LessThan::new(Variable::new("x"), Number(60_001)),
            Assign::new("x", Add::new(Variable::new("x"), Number(2))),
        ),
    );
    println!("{}", seq.inspect());
    println!("{}", seq.evaluate(&Environment::empty()));
}
