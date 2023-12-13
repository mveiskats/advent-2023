use std::io;
use std::cmp;
use ndarray::Array2;

fn main() {
    let lines: Vec<String> = io::stdin().lines()
        .map(|line| line.unwrap())
        .collect();

    let mut patterns: Vec<Vec<String>> = vec![];
    patterns.push(vec![]);

    for line in lines {
        if line.len() == 0 {
            patterns.push(vec![]);
        } else {
            let last = patterns.len() - 1;
            patterns[last].push(line);
        }
    }

    let mut result = 0;

    for pattern in patterns {
        let rows = pattern.len();
        let cols = pattern[0].len();

        let mut map: Array2<bool> = Array2::from_elem((cols, rows), false);

        for (y, line) in pattern.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                map[(x, y)] = ch == '#';
            }
        }

        for y in 1..rows {
            let bound = cmp::min(y, rows - y);
            let matches: Vec<usize> = (0..bound).map(|n|{
                (0..cols).filter(|&x| map[(x, y + n)] == map[(x, y - n - 1)]).count()
            }).collect();

            let full = matches.iter().filter(|&&m| m == cols).count();
            let partial = matches.iter().filter(|&&m| m == cols - 1).count();

            if full == bound - 1 && partial == 1 { result += y * 100; }
        }

        for x in 1..cols {
            let bound = cmp::min(x, cols - x);
            let matches: Vec<usize> = (0..bound).map(|n|{
                (0..rows).filter(|&y| map[(x + n, y)] == map[(x - n - 1, y)]).count()
            }).collect();

            let full = matches.iter().filter(|&&m| m == rows).count();
            let partial = matches.iter().filter(|&&m| m == rows - 1).count();

            if full == bound - 1 && partial == 1 { result += x; }
        }
    }

    println!("{result}");
}
