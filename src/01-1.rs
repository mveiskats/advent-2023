use std::io;

fn main() {
    let total: u32 = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| {
            let first = line.chars()
                .find_map(|ch| ch.to_digit(10))
                .expect("no digits found");

            let last = line.chars().rev()
                .find_map(|ch| ch.to_digit(10))
                .expect("no digits found");

            first * 10 + last
        }).sum();

    println!("{total}");
}
