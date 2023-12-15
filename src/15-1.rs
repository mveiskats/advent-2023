use std::io;

fn main() {
    let input = io::stdin().lines().next().unwrap().unwrap();
    let result: usize = input.split(',').map(|s| {
        s.chars().fold(0, |acc, ch| (acc + (ch as usize)) * 17 % 256)
    }).sum();

    println!("{result}");
}
