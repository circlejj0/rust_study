use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let year = iter.next().unwrap();
    if year % 4 == 0 && year % 100 != 0 {
        println!("1");
    } else if year % 400 == 0 {
        println!("1");
    } else {
        println!("0");
    }
}
