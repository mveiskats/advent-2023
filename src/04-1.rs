use std::io;

fn main() {
    let score: usize = io::stdin().lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let numbers = line.split(": ").collect::<Vec<&str>>()[1];
            let (winning, owned) = numbers.split_once(" | ").unwrap();
            let winning: Vec<usize> = winning.split(' ').filter(|s| s.len() > 0).map(|s| s.parse::<usize>().unwrap()).collect();
            let owned: Vec<usize> = owned.split(' ').filter(|s| s.len() > 0).map(|s| s.parse::<usize>().unwrap()).collect();

            winning.iter().filter(|&n| owned.iter().any(|m| n == m)).count()
        }).map(|matching| {
            if matching > 0 {
                (1..matching).fold(1, |mult, _| mult * 2)
            } else {
                0
            }
        }).sum();

    println!("{score}");
}
