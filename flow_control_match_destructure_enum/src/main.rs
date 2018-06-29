#![allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    // println!("Hello, world!");

    // let color = Color::RGB(122, 17, 40);

    let color = Color::CMYK(122,12,2,1);

    println!("What color is it?");
    match color {
        Color::Red => println!("The color is red!"),
        Color::Blue => println!("The color is blue!"),
        Color::Green => println!("The color is green!"),
        Color::RGB(r, g, b) => 
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, mangento: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magento: {}, yellow: {}, key: {}!", c, m, y, k),
    }
}
