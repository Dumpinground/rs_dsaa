// Results and Options

#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E),
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        return Res::Error("Cannot divide by zero".to_string());
    }

    Res::Thing(a / b)
}

fn main() {
    let (a, b) = (divide(10, 5), divide(10, 0));

    match a {
        Res::Thing(v) => println!("val = {}", v),
        _ => {}
    }

    println!("{:?} {:?}", a, b);
}
