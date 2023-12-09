use std::io;
use std::ops::Range;
use regex::Regex;

struct Number {
    pub value: usize,
    pub row: usize,
    pub cols: Range<usize>
}

fn main() {
    let rx = Regex::new(r"[0-9]+").unwrap();
    let mut numbers: Vec<Number> = vec![];
    let board: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();

    for (row, line) in board.iter().enumerate() {
        numbers.extend(rx.find_iter(line).map(|m| {
            Number {
                value: m.as_str().parse::<usize>().unwrap(),
                row,
                cols: m.start()..m.end()
            }
        }));
    }

    let mut result = 0;

    for (row, line) in board.iter().enumerate() {
        line.chars().enumerate()
            .filter(|(_col, ch)| *ch == '*')
            .for_each(|(col, _ch)| {
                let touching: Vec<usize> = numbers.iter().filter(|num| {
                    (num.row + 1 >= row) && (num.row <= row + 1) &&
                        (num.cols.start <= col + 1) && (num.cols.end >= col)
                }).map(|num| num.value).collect();

                if touching.len() == 2 { result += touching[0] * touching[1] }
            })
    }

    println!("{result}");
}
