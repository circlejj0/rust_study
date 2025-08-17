use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let x: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut total = 0;

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace().map(|v| v.parse::<i32>().unwrap());

        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        total += a * b;
    }
    if total == x {
        println!("Yes");
    } else {
        println!("No");
    }
}
