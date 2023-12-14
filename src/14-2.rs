use std::io;
use ndarray::{ s, Array2, ArrayViewMut2, Axis };

fn roll(mut board: ArrayViewMut2<char>) {
    let (cols, rows) = board.dim();

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
}

fn cycle(board: &mut Array2<char>) {
    roll(board.view_mut());
    roll(board.view_mut().reversed_axes());

    let mut view = board.view_mut();
    view.invert_axis(Axis(1));
    roll(view);

    let mut view = board.view_mut().reversed_axes();
    view.invert_axis(Axis(1));
    roll(view);
}

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

    let mut history: Vec<Array2<char>> = vec![];

    for _ in 0..1_000_000_000 {
        history.push(board.clone());
        cycle(&mut board);

        if let Some(start) = history.iter().position(|h| h == board) {
            history.push(board);
            let end = history.len() - 1;
            let len = end - start;

            println!("Found loop {start}..={end}");

            let final_pos = ((1_000_000_000 - start)) % len + start;
            board = history[final_pos].clone();

            break
        }
    }

    let load: usize = (0..cols).map(|y| {
        let weight = rows - y;
        weight * board.slice(s![.., y]).iter().filter(|&&ch| ch == 'O').count()
    }).sum();

    println!("{load}");
}
