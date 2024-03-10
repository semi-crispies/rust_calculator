use std::io;

pub fn read_u8() -> u8 {
    let input = io::stdin().lines().next().unwrap().unwrap();
    input.parse().unwrap()
}

pub fn read_u16() -> u16 {
    let input = io::stdin().lines().next().unwrap().unwrap();
    input.parse().unwrap()
}

pub fn read_f32() -> f32 {
    let input = io::stdin().lines().next().unwrap().unwrap();
    input.parse().unwrap()
}