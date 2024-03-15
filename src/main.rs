extern crate tools;
extern crate bmi_calculator;

fn main() {
    println!("What do you want?\n 1. Infinite Calculator\n 2. IMC Calculator");
    // Init "input" variable to 0 at start
    let mut input: i32 = 0;
    // Step to compare VALID_NUMBER with user input value
    while !VALID_NUMBER.contains(&input) {
        input = tools::input_checker_i32();
        match input {
            1 => println!("Infinite Calculator"),
            2 => bmi_calculator::main(),
            _n => println!("Please, specify your request"),
        };
    };
}

const VALID_NUMBER: [i32; 2] = [1, 2];