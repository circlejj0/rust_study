use std::io;
use std::cmp::max;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let one = iter.next().unwrap();
    let two = iter.next().unwrap();
    let three = iter.next().unwrap();

    if one == two && two == three {
        println!("{}", 10000 + one * 1000);
    } else if one == two {
        println!("{}", 1000 + one * 100);
    } else if two == three {
        println!("{}", 1000 + two * 100);
    } else if one == three {
        println!("{}", 1000 + one * 100);
    } else {
        let biggest = max(one, max(two, three));
        println!("{}", biggest * 100);
    }
}
