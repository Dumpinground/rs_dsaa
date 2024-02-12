// Build complex structures with Struct and Enum

#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    favor_color: Color,
}

#[derive(Debug)]
pub enum Color {
    Red(String),
    Green,
    Blue,
}

impl Person {
    pub fn print(&self) -> String {
        format!(
            "name is {}, age is {}, has {} children.",
            self.name, self.age, self.children
        )
    }
}

fn main() {
    let p = Person {
        name: "matt".into(),
        age: 35,
        children: 4,
        favor_color: Color::Green,
    };

    println!("Hello, {}", p.print());
    println!("Hello, {:?}", p);

    let c = Color::Red("hello".into());

    match c {
        Color::Red(s) => println!("It's red {}", s),
        Color::Green => println!("It's green"),
        Color::Blue => println!("It's blue"),
    }
}
