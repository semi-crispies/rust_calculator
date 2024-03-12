use std::io;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn main_input_checker() -> i32 {
    // Init input variable
    let mut input_string = String::new();

    // Display input
    match io::stdin().read_line(&mut input_string) {
        Ok(_) => { let input_string = input_string.trim(); },
        Err(error) => println!("error: {}", error),
    };

    // Step to convert input to Integer
    let mut input_number: i32 = 0;
    match input_string.trim().parse::<i32>() {
        Ok(_) => { let input_number: i32 = input_string.trim().parse::<i32>().unwrap(); },
        Err(error) => {
            println!("Not a Number : {}", error);
            return 0;
        },
    };

    // Step to compare VALID_NUMBER with user input value
    const VALID_NUMBER: [i32; 2] = [1, 2];
    if !VALID_NUMBER.contains(&input_number) { println!("Wrong number, try a valid number, please..."); };

    // If everything ok RETURN input_number
    input_number
}

pub fn read_u16() -> u16 {
    let input = io::stdin().lines().next().unwrap().unwrap();
    input.parse().unwrap()
}

pub fn read_f32() -> f32 {
    let input = io::stdin().lines().next().unwrap().unwrap();
    input.parse().unwrap()
}