use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let mut output = String::new();

    while let (Some(a), Some(b)) = (iter.next(), iter.next()) {
        output.push_str(&(a + b).to_string());
            output.push('\n');
    }

    print!("{}", output);
}
