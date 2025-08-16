use std::io;

fn main() {
    let mut input_one = String::new();
    let mut input_two = String::new();
    io::stdin().read_line(&mut input_one).unwrap();
    io::stdin().read_line(&mut input_two).unwrap();

    let mut iter_one = input_one.split_whitespace().map(|x| x.parse::<u32>().unwrap());
    let mut iter_two = input_two.split_whitespace().map(|x| x.parse::<u32>().unwrap());
    let a: u32 = iter_one.next().unwrap();
    let b: u32 = iter_two.next().unwrap();
    let c = b % 10;
    let d = (b / 10) % 10;
    let e = b / 100;

    println!("{}", a * c);
    println!("{}", a * d);
    println!("{}", a * e);
    println!("{}", a * b);
}
