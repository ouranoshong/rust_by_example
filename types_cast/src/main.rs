#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // let integer: u8 = decimal;

    let integer = decimal as u8;

    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    

    println!("Hello, world!");
}
