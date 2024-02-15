// Results and Options

#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E),
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Cannot divide by zero".to_string());
    }

    Ok(a / b)
}

fn main() {
    let (a, b) = (divide(10, 5), divide(10, 0));

    if let Ok(v) = a {
        println!("val = {}", v);
    }

    println!("{:?} {:?}", a, b);
}
