use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let mut output = String::new();

    loop {
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();

        if a == 0 && b == 0 {
            break;
        }

        output.push_str(&(a + b).to_string());
        output.push('\n');
    }

    println!("{}", output);
}
