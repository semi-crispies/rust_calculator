use std::io;

pub fn input_checker_i32() -> i32 {
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

pub fn input_checker_f32() -> f32 {
    // Init input variable as a String
    let mut input_string = String::new();

    // Display input and replace "," by "." if Ok
    match io::stdin().read_line(&mut input_string) {
        Ok(_) => { input_string = input_string.replace(",", "."); },
        Err(error) => println!("error: {}", error),
    };

    // Step to convert input to Float
    let mut input_number: f32 = 0.0;
    match input_string.trim().parse::<f32>() {
        Ok(_) => { input_number = input_string.trim().parse::<f32>().unwrap(); },
        Err(error) => {
            println!("Not a Number : {}", error);
        },
    };

    // If everything ok RETURN input_number
    input_number
}