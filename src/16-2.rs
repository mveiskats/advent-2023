use std::io;
use ndarray::{ s, Array2 };
use rayon::prelude::*;

struct Board {
    tiles: Array2<char>,
    visited_dirs: Array2<Vec<(isize, isize)>>
}

impl Board {
    fn visit(&mut self, (x, y): (isize, isize), (dx, dy): (isize, isize)) {
        let (cols, rows) = self.tiles.dim();

        if dx == 0 && dy == 0 { panic!("stalled") }
        if 0 > x || x >= (cols as isize) || 0 > y || y >= (rows as isize) { return }

        let ux = x as usize;
        let uy = y as usize;

        if self.visited_dirs[(ux, uy)].iter().any(|&(vdx, vdy)| vdx == dx && vdy == dy) { return }

        self.visited_dirs[(ux, uy)].push((dx, dy));

        match self.tiles[(ux, uy)] {
            '.' => {
                self.visit((x + dx, y + dy), (dx, dy))
            },
            '/' => {
                let (ndx, ndy) = (-dy, -dx);
                self.visit((x + ndx, y + ndy), (ndx, ndy));
            },
            '\\' => {
                let (ndx, ndy) = (dy, dx);
                self.visit((x + ndx, y + ndy), (ndx, ndy));
            },
            '|' => {
                if dy == 0 {
                    let (ndx, ndy) = (dy, dx);
                    self.visit((x + ndx, y + ndy), (ndx, ndy));
                    let (ndx, ndy) = (dy, -dx);
                    self.visit((x + ndx, y + ndy), (ndx, ndy));
                } else {
                    self.visit((x + dx, y + dy), (dx, dy));
                }
            },
            '-' => {
                if dx == 0 {
                    let (ndx, ndy) = (dy, dx);
                    self.visit((x + ndx, y + ndy), (ndx, ndy));
                    let (ndx, ndy) = (-dy, dx);
                    self.visit((x + ndx, y + ndy), (ndx, ndy));
                } else {
                    self.visit((x + dx, y + dy), (dx, dy));
                }
            },
            _ => panic!("unknown char")
        }
    }

    fn visited_count(&self) -> usize {
        self.visited_dirs.slice(s![.., ..]).iter().filter(|&v| v.len() > 0).count()
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .collect();

    let rows = lines.len();
    let cols = lines.len();

    let mut tiles: Array2<char> = Array2::from_elem((cols, rows), '.');
    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            tiles[(x, y)] = ch;
        })
    });

    let result: usize = (0..cols).into_par_iter().map(|x| ((x, 0), (0, 1)))
        .chain((0..cols).into_par_iter().map(|x| ((x, rows - 1), (0, -1))))
        .chain((0..rows).into_par_iter().map(|y| ((0, y), (1, 0))))
        .chain((0..rows).into_par_iter().map(|y| ((cols - 1, y), (-1, 0))))
        .map(|((x, y), (dx, dy))| {
            let mut board = Board {
                tiles: tiles.clone(),
                visited_dirs: Array2::from_elem((cols, rows), vec![])
            };
            board.visit((x as isize, y as isize), (dx, dy));
            board.visited_count()
        }).max().unwrap();

    println!("{result}");
}
