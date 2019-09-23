use uc::small_step::{Value::Number, *};

fn main() {
    let mut machine = Machine::new(Sequence::new(
        Assign::new("x", Add::new(Number(3), Number(5))),
        Assign::new("y", Add::new(Number(3), Number(1))),
    ));
    machine.run();
}
