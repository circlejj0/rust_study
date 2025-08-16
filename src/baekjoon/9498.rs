use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let score = iter.next().unwrap();
    if 90 <= score && score <= 100 {
        println!("A");
    } else if 80 <= score && score <= 89 {
        println!("B");
    } else if 70 <= score && score <= 79 {
        println!("C");
    } else if 60 <= score && score <= 69 {
        println!("D");
    } else {
        println!("F");
    }
}
