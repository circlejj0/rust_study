use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    println!("{}", result);
}
