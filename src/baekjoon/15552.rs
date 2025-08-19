use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());

    let t: i32 = iter.next().unwrap();

    let mut output = String::new();

    for _ in 0..t {
        let a: i32 = iter.next().unwrap();
        let b: i32 = iter.next().unwrap();

        output.push_str(&(a + b).to_string());
        output.push('\n');
    }
    print!("{}", output);
}
