extern crate tools;

pub fn main() {
    println!("Welcome into the Body Mass Index Calculator");
    let weight = weight();
    let height = height();
    println!("Your Body Mass Index is {}", weight / (height * height));
}

fn weight() -> f32 {
    println!("What is your weight in kg with decimal (ex: 70.0 for 70kg) ?");
    let weight = tools::read_f32();
    println!("{} kg", weight);
    return weight;
}

fn height() -> f32 {
    println!("What is your height in meter (ex: 1.8 for 1meter80) ?");
    let height = tools::read_f32();
    println!("{} m", height);
    return height;
}