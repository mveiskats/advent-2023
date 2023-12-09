use std::io;

fn is_part(board: &Vec<Vec<char>>, row: isize, col: isize) -> bool {
    if row < 0 || col < 0 { return false }

    let row = row as usize;
    let col = col as usize;

    if row >= board.len() || col >= board[0].len() { return false }

    let ch = board[row][col];

    !(ch.is_digit(10) || ch == '.')
}

fn is_touching(board: &Vec<Vec<char>>, row: isize, col: isize) -> bool {
    is_part(&board, row - 1, col - 1) ||
        is_part(&board, row - 1, col) ||
        is_part(&board, row - 1, col + 1) ||
        is_part(&board, row, col - 1) ||
        is_part(&board, row, col + 1) ||
        is_part(&board, row + 1, col - 1) ||
        is_part(&board, row + 1, col)||
        is_part(&board, row + 1, col + 1)
}

fn main() {
    let board: Vec<Vec<char>> = io::stdin().lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let rows: isize = board.len() as isize;
    let cols: isize = board[0].len() as isize;

    let mut result = 0;

    for row in 0..rows {
        let mut number: usize = 0;
        let mut touching = false;

        for col in 0..cols {
            let ch = board[row as usize][col as usize];

            if let Some(d) = ch.to_digit(10) {
                number = number * 10 + d as usize;
                touching |= is_touching(&board, row, col);
            } else {
                if number > 0 && touching { result += number; }
                number = 0;
                touching = false;
            }
        }
        if number > 0 && touching { result += number; }
    }

    println!("{result}");
}
