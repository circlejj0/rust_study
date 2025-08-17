use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let n = iter.next().unwrap();

    for i in 1..10 {
        println!("{} * {} = {}", n, i, n*i);
    }
}
