

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8}
}

fn enums() {
    let c: Color = Color::Cmyk {cyan:1, magenta: 2, yellow: 3, black: 4};
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("Black"),
        Color::RgbColor(r, g, b) => println!("{}, {}, {}", r, g, b),
        Color::Cmyk {cyan:_, magenta: _, yellow:_, black: 4} => println!("Cymk"),
        _ => println!("Catch all")
    }
}

fn main() {
    enums();
}
