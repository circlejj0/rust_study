use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    for _i in 0..n {
        let result = "*".repeat(_i + 1);
        println!("{}", result);
    }
}
