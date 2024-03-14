extern crate tools;
use colored::Colorize;

pub fn main() {
    println!("{}", "Welcome into the Body Mass Index Calculator".bold().yellow());
    let weight: f32 = weight();
    let height: f32 = height();
    let bmi: f32 = body_mass_index_calculator(weight, height);
    println!(" - WEIGHT: {} kg\n - HEIGHT: {} m", weight , height);
    println!("Your Body Mass Index is {:.4}", bmi.to_string().bold().yellow());
}

fn weight() -> f32 {
    let mut weight: f32 = 0.0;
    while weight < 50.0 || weight > 200.0 {
        println!(" - What is your {} in kg (ex: 70.5 for 70.5kg) ?", "WEIGHT".yellow());
        println!("{}", " * Your weight (in kilogram) must be between 50.0 and 200.0".red());
        weight = tools::input_checker_f32();
    }
    return weight;
}

fn height() -> f32 {
    let mut height: f32 = 0.0;
    while height < 1.0 || height > 3.0 {
        println!(" - What is your {} in meter (ex: 1.8 for 1meter80) ?", "HEIGHT".yellow());
        println!("{}", " * Your height (in meter) must be between 1.0 and 3.0".red());
        height = tools::input_checker_f32();
    }
    return height;
}

fn body_mass_index_calculator(weight: f32, height: f32) -> f32 {
    return weight / ( height * height );
}
