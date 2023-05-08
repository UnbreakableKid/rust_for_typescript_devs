// fn main() {
//     let list: Vec<i32> = vec![0, 1, 2, 3];

//     let list: Vec<_> = list.iter().map(|x| x + 1).collect();

//     print!("{:?}", list);
// }

// fn main() {
//     let file = std::fs::read_to_string("lines");

//     if let Ok(x) = file {
//         x.lines()
//             .enumerate()
//             .filter(|(indice, _)| indice % 2 == 0)
//             .skip(2)
//             .take(2)
//             .for_each(|(_, line)| println!("{:?}", line))
//     }
// }

use std::fmt::Display;

enum Color {
    Red,
    Green,
    Blue,
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
            Color::Red => return false,
            Color::Green => return false,
            Color::Blue => return true,
            Color::Yellow => return true,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Blue => write!(f, "blue"),
            Color::Red => write!(f, "red"),
            Color::Green => write!(f, "green"),
            Color::Yellow => write!(f, "yellow"),
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Blue => println!("{}", Color::Blue),
        Color::Red => println!("{}", Color::Red),
        Color::Green => println!("{}", Color::Green),
        Color::Yellow => println!("{}", Color::Yellow),
    }
}

fn main() {
    let foo = Color::Green;

    foo.is_green_parts();
}
