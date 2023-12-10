use std::io;

fn main() {
    let total: u32 = io::stdin().lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let first = line.chars().find(|ch| ch.is_digit(10)).unwrap().to_digit(10).unwrap();
            let last = line.chars().rev().find(|ch| ch.is_digit(10)).unwrap().to_digit(10).unwrap();

            first * 10 + last
        }).sum();

    println!("{total}");
}
