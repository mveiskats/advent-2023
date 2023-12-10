use std::io;

fn main() {
    let matches: Vec::<usize> = io::stdin().lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let numbers = line.split(": ").collect::<Vec<&str>>()[1];
            let (winning, owned) = numbers.split_once(" | ").unwrap();
            let winning: Vec<usize> = winning.split(' ').filter(|s| s.len() > 0).map(|s| s.parse::<usize>().unwrap()).collect();
            let owned: Vec<usize> = owned.split(' ').filter(|s| s.len() > 0).map(|s| s.parse::<usize>().unwrap()).collect();

            winning.iter().filter(|&n| owned.iter().any(|m| n == m)).count()
        }).collect();

    let mut cards: Vec<Option<usize>> = vec![None; matches.len()];

    for i in (0..cards.len()).rev() {
        let mut subtotal = matches[i];
        if subtotal > 0 {
            subtotal += cards[(i + 1)..=(i + subtotal)].iter()
                .map(|n| n.unwrap())
                .sum::<usize>();
        }

        cards[i] = Some(subtotal);
    }

    let total: usize = cards.into_iter().map(|n| n.unwrap()).sum::<usize>() + matches.len();

    println!("{total}");
}
