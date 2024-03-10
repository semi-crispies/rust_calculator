extern crate tools;
extern crate imc_calculator;

fn main() {
    println!("What do you want?\n 1. Infinite Calculator\n 2. IMC Calculator");
    let input = tools::read_u8();
    match input {
        1 => println!("Infinite Calculator"),
        2 => imc_calculator::main(),
        n => println!("T'es le roi des CONNARDS !!!!!!!"),
    }
}
