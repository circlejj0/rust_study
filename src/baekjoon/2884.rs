use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let hour = iter.next().unwrap();
    let min = iter.next().unwrap();

    if (min-45) >= 0 {
        println!("{} {}", hour, min - 45);
    } else if (min-45) < 0 {
        if (hour-1) < 0 {
            println!("23 {}", min + 15);
        } else {
            println!("{} {}", hour - 1, min + 15);
        }
    } 
}
