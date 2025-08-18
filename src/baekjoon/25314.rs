use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    let result = "long ".repeat(n / 4);
    println!("{}int", result);
}
