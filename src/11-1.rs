use std::io;
use ndarray::{Array2, s};

fn main() {
    let lines: Vec<String> = io::stdin().lines()
        .map(|line| line.unwrap())
        .collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut map: Array2<bool> = Array2::from_elem((cols, rows), false);
    let mut galaxies: Vec<(usize, usize)> = vec![];

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                map[(x, y)] = true;
                galaxies.push((x, y));
            }
        }
    }

    let empty_cols: Vec<usize> = (0..cols).filter(|&x| {
        map.slice(s![x, ..]).iter().all(|b| !b)
    }).collect();

    let empty_rows: Vec<usize> = (0..rows).filter(|&y| {
        map.slice(s![.., y]).iter().all(|b| !b)
    }).collect();

    let result: usize = galaxies[..(galaxies.len() - 1)].iter().enumerate().map(|(i, (x1, y1))| {
        galaxies[(i + 1)..].iter().map(|(x2, y2)| {
            let dx = (*x1 as isize - *x2 as isize).abs() as usize +
                empty_cols.iter().filter(|ex| (x1 < *ex && *ex < x2) || (x2 < *ex && *ex < x1)).count();
            let dy = (*y1 as isize - *y2 as isize).abs() as usize +
                empty_rows.iter().filter(|ey| (y1 < *ey && *ey < y2) || (y2 < *ey && *ey < y1)).count();

            dx + dy
        }).sum::<usize>()
    }).sum();

    println!("{result}");
}
