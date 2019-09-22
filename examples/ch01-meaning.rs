use uc::small_step::{Value::Number, *};

fn main() {
    let mut machine = Machine::new(Assign::new("x", Add::new(Number(3), Number(5))));
    machine.run();
}
