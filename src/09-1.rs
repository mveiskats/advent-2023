use std::io;

fn prediction(history: &[isize]) -> isize {
    let diff: Vec<isize> = history[..history.len() - 1].iter()
        .zip(history[1..].iter())
        .map(|(a, b)| b - a)
        .collect();

    *history.last().unwrap() +
        if diff.iter().all(|&n| n == 0) {
            0
        } else {
            prediction(&diff[..])
        }
}

fn main() {
    let result: isize = io::stdin().lines().map(|line| {
        let line = line.unwrap();

        let history: Vec<isize> = line.split(" ")
            .map(|s| s.parse::<isize>().unwrap())
            .collect();

        prediction(&history[..])
    }).sum();

    println!("{result}");
}
