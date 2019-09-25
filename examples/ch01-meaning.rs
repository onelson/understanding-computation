use uc::small_step::{Value::*, *};

fn main() {
    let mut machine = Machine::new(If::new(
        Boolean(false),
        Assign::new("x", Add::new(Number(3), Number(5))),
        Assign::new("y", Add::new(Number(3), Number(1))),
    ));
    machine.run();
}
