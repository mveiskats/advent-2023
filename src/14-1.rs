use std::io;
use ndarray::{ s, Array2 };

fn main() {
    let lines: Vec<String> = io::stdin().lines()
        .map(|line| line.unwrap())
        .collect();

    let rows = lines.len();
    let cols = lines.len();

    let mut board: Array2<char> = Array2::from_elem((cols, rows), '.');
    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            board[(x, y)] = ch;
        })
    });

    // Roll the rocks
    for x in 0..cols {
        for y in 0..rows {
            if board[(x, y)] == 'O' {
                let i = board.slice(s![x, 0..y;-1]).iter().take_while(|&&ch| ch == '.').count();

                if i > 0 {
                    board[(x, y - i)] = 'O';
                    board[(x, y)] = '.';
                }
            }
        }
    }

    // for y in 0..rows {
    //     for x in 0..cols {
    //         print!("{}", board[(x, y)]);
    //     }
    //     println!();
    // }

    let load: usize = (0..cols).map(|y| {
        let weight = rows - y;
        weight * board.slice(s![.., y]).iter().filter(|&&ch| ch == 'O').count()
    }).sum();

    println!("{load}");
}
