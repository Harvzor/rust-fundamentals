use std::mem;

enum Color {
    Red,
    Green,
    Blue,
    // Tuple
    RgbColor(u8, u8, u8),
    CmykColor { cyan: u8, magenta: u8, yellow: u8, black: u8 }
}

fn main() {
    // let c = Color::RgbColor(10, 0, 0);
    let c = Color::CmykColor {
        cyan: 0,
        magenta: 0,
        yellow: 0,
        black: 255,
    };

    match c
    {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        Color::CmykColor { cyan: _, magenta: _, yellow: _, black: 255} => println!("black"),
        _ => ()
    }
}
