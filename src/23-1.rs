use std::io;
use ndarray::{ s, Array2 };

#[derive(PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Board {
    tiles: Array2<char>,
    steps: Array2<usize>,
}

impl Board {
    fn visit(&mut self, (x, y): (usize, usize), step: usize, dir: Dir) {
        let rows = self.tiles.dim().1;
        let tile = self.tiles[(x, y)];

        if tile == '#' { return }
        if self.steps[(x, y)] > step { return }

        // Can't go up slope
        match dir {
            Dir::Up => if tile == 'v' { return },
            Dir::Down => if tile == '^' { return },
            Dir::Left => if tile == '>' { return },
            Dir::Right => if tile == '<' { return },
        }

        self.steps[(x, y)] = step;

        // Reached exit
        if y == rows - 1 { return }

        if dir != Dir::Down { self.visit((x, y - 1), step + 1, Dir::Up) }
        if dir != Dir::Up { self.visit((x, y + 1), step + 1, Dir::Down) }
        if dir != Dir::Right { self.visit((x - 1, y), step + 1, Dir::Left) }
        if dir != Dir::Left { self.visit((x + 1, y), step + 1, Dir::Right) }
    }

    fn longest_hike(&mut self) -> usize {
        let (_cols, rows) = self.tiles.dim();
        let start_x = self.tiles.slice(s![.., 0]).iter().position(|&ch| ch == '.').expect("no start");
        let end_x = self.tiles.slice(s![.., -1]).iter().position(|&ch| ch == '.' ).expect("no end");

        self.visit((start_x, 1), 1, Dir::Down);

        self.steps[(end_x, rows - 1)]
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut tiles: Array2<char> = Array2::from_elem((cols, rows), '.');

    for (y, line) in lines.into_iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            tiles[(x, y)] = ch;
        }
    }

    let steps: Array2<usize> = Array2::<usize>::zeros((cols, rows));

    let mut board = Board { tiles, steps };

    println!("{}", board.longest_hike());
}
