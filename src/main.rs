extern crate tools;
extern crate bmi_calculator;

fn main() {
    println!("What do you want?\n 1. Infinite Calculator\n 2. IMC Calculator");
    let input = tools::main_input_checker();
    match input {
        1 => println!("Infinite Calculator"),
        2 => bmi_calculator::main(),
        _n => println!("ERROR"),
    }
}
