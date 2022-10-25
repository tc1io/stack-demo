use std::fmt;
use std::fmt::Formatter;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let cirle = Circle { radius: 10 };
    println!("{}", cirle.to_string());

    let parce: i32 = "5".parse().unwrap();
    let t_parse = "10".parse::<i32>().unwrap();

    let sum = parce + t_parse;
    println!("Sun is {:?}", sum);
}