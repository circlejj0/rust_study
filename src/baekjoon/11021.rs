use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());

    let t: i32 = iter.next().unwrap();

    for r in 0..t {
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();

        println!("Case #{}: {}", 1 + r, a + b);
    }
}
