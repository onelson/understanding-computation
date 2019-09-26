use uc::small_step::{Value::*, *};

fn main() {
    let mut machine = Machine::new(Sequence::new(
        Assign::new("x", Number(0)),
        While::new(
            LessThan::new(Variable::new("x"), Number(5)),
            Assign::new("x", Add::new(Variable::new("x"), Number(2))),
        ),
    ));
    machine.run();
}
