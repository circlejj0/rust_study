use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();

    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", a * b);
    println!("{}", a / b);
    println!("{}", a % b);
}
