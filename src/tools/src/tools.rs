use std::io;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn read_u8() {
    let mut input = String::new();

    let input = match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("success: {}", input);
        },
        Err(error) => println!("error: {}", error),
    };

    // 'input' is no more a string but a unit ()
    println!("{}", type_of(input));       // -> ()
    println!("{}", type_of("jakigeon"));  // -> &str
    // step to convert "input" into String

    let input = "9";
    let input = match input.trim().parse::<i32>() {
        Ok(_) => println!("top"),
        Err(error) => println!("NaN : {}", error),
    };

    // 'input' is no more a string but a unit ()
    println!("{}", type_of(input));       // -> ()
    println!("{}", type_of(7));           // -> i32
    // step to convert "input" into String

    let input = 2;
    const VALID_NUMBER: [u8; 2] = [1, 2];
    if !VALID_NUMBER.contains(&input) { println!("error"); };
}

pub fn read_u16() -> u16 {
    let input = io::stdin().lines().next().unwrap().unwrap();
    input.parse().unwrap()
}

pub fn read_f32() -> f32 {
    let input = io::stdin().lines().next().unwrap().unwrap();
    input.parse().unwrap()
}