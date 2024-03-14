use std::io;

pub fn main_input_checker() -> i32 {
    // Init input variable
    let mut input_string = String::new();

    // Display input
    match io::stdin().read_line(&mut input_string) {
        Ok(_) => {},
        Err(error) => println!("error: {}", error),
    };

    // Step to convert input to Integer
    let mut input_number: i32 = 0;
    match input_string.trim().parse::<i32>() {
        Ok(_) => { input_number = input_string.trim().parse::<i32>().unwrap(); },
        Err(error) => {
            println!("Not a Number : {}", error);
        },
    };

    // If everything ok RETURN input_number
    input_number
}

pub fn read_f32() -> f32 {
    let input = io::stdin().lines().next().unwrap().unwrap();
    input.parse().unwrap()
}