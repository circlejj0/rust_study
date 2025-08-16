use std::io;

fn main() {
    let mut input_one = String::new();
    let mut input_two = String::new();
    io::stdin().read_line(&mut input_one).unwrap();
    io::stdin().read_line(&mut input_two).unwrap();

    let mut iter_one = input_one.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let mut iter_two = input_two.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let hour = iter_one.next().unwrap();
    let min = iter_one.next().unwrap();
    let time = iter_two.next().unwrap();

    let hour = hour + (time/60);
    let hour = hour % 24;
    let time = time % 60;

    if (min + time) < 60 {
        println!("{} {}", hour, min + time);
    } else if (min + time) >= 60 {
        if hour + ((min + time) / 60) > 23 {
            println!("{} {}", hour - 23, min + time -60 * ((min + time) / 60));
        } else {
            println!("{} {}", hour + ((min + time) / 60), min + time - 60 * ((min + time) / 60));
        }
    }
}
