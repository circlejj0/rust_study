use std::io;

fn main() {
    let mut input_one = String::new();
    let mut input_two = String::new();
    io::stdin().read_line(&mut input_one).unwrap();
    io::stdin().read_line(&mut input_two).unwrap();

    let mut iter_one = input_one.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let mut iter_two = input_two.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let quad_one = iter_one.next().unwrap();
    let quad_two = iter_two.next().unwrap();

    if quad_one > 0 && quad_two > 0 {
        println!("1");
    } else if quad_one < 0 && quad_two > 0 {
        println!("2");
    } else if quad_one < 0 && quad_two < 0 {
        println!("3");
    } else if quad_one > 0 && quad_two < 0 {
        println!("4")
    }
}
