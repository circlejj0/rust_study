use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace().map(|x| x.parse::<i64>().unwrap());
    let n = iter.next().unwrap();
    let m = iter.next().unwrap();
    let minus = n - m;
    let domain: i64 = minus.abs();

    println!("{}", domain);
}
