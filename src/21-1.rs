use std::io;
use ndarray::{ s, Array2 };

const STEP_TARGET: i8 = 64;

fn main() {
    let lines: Vec<String> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut walls: Array2<bool> = Array2::from_elem((cols, rows), false);

    let mut start_x: usize = 0;
    let mut start_y: usize = 0;

    for (y, line) in lines.into_iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    start_x = x;
                    start_y = y;
                },
                '#' => walls[(x, y)] = true,
                _ => ()
            }
        }
    }

    let mut current_step: i8 = 0;
    let mut steps: Array2<Option<i8>> = Array2::from_elem((cols, rows), None);
    let mut from: Vec<(usize, usize)> = vec![(start_x, start_y)];

    while current_step <= STEP_TARGET {
        let mut to: Vec<(usize, usize)> = vec![];
        for (x, y) in from {
            if walls[(x, y)] { continue }
            if steps[(x, y)] != None { continue }

            steps[(x, y)] = Some(current_step);

            if x > 0 { to.push((x - 1, y)) }
            if y > 0 { to.push((x, y - 1)) }
            if x < cols - 1 { to.push((x + 1, y)) }
            if y < rows - 1 { to.push((x, y + 1)) }
        }
        from = to;
        current_step += 1;
    }

    let result = steps.slice(s![.., ..]).iter().flatten().filter(|&step| step % 2 == 0).count();
    println!("{result}");
}
