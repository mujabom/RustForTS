use crate::Color::Red;

enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Green => false,
            Color::Red => false,
            Color::Blue => true,
            Color::Yellow => true,
        }
    }
}
fn main() {
    print_color(Red);
    let foo = Color::Blue;
    println!("{}",foo.is_green_parts())
}

fn print_color(color: Color) -> () {
    match color {
        Color::Red => {
            println!("Red")
        }
        Color::Blue => {
            println!("blue")
        }
        Color::Green => {
            println!("green")
        }

        Color::Yellow => {
            println!("Yellow")
        }
    }
}

