extern crate tools;
extern crate bmi_calculator;
extern crate infinite_calculator;
use colored::Colorize;

fn main() {
    println!("{}\n {} Infinite Calculator\n {} IMC Calculator", "What do you want?".bold(), "1.".yellow(), "2.".yellow());
    // Init "input" variable to 0 at start
    let mut input: i32 = 0;
    // Step to compare VALID_NUMBER with user input value
    while !VALID_NUMBER.contains(&input) {
        input = tools::input_checker_i32();
        match input {
            1 => infinite_calculator::main(),
            2 => bmi_calculator::main(),
            _n => println!("Please, specify your request"),
        };
    };
}

const VALID_NUMBER: [i32; 2] = [1, 2];