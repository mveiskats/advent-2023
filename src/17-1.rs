use std::io;
use std::collections::HashMap;
use ndarray::{ s, Array2 };

struct Board {
    tiles: Array2<u8>,
    mems: Array2<HashMap<(i16, i16, u8), usize>>,
    pub min_cost: usize
}

impl Board {
    fn enter(&mut self, (x, y): (i16, i16), (dx, dy): (i16, i16), path_cost: usize, straight: u8) {
        let (cols, rows) = self.tiles.dim();

        if 0 > x || x >= (cols as i16) || 0 > y || y >= (rows as i16) { return }

        let ux = x as usize;
        let uy = y as usize;

        let path_cost = path_cost + self.tiles[(ux, uy)] as usize;

        if path_cost > self.min_cost { return }

        let tile_mem = &mut self.mems[(ux, uy)];
        let key = (dx, dy, straight);

        if let Some(mem_cost) = tile_mem.get_mut(&key) {
            if path_cost >= *mem_cost { return }
            *mem_cost = path_cost
        } else {
            tile_mem.insert(key, path_cost);
        }

        if ux == cols - 1 && uy == rows - 1 {
            if path_cost < self.min_cost {
                self.min_cost = path_cost;
            }
            return
        }

        self.enter((x + dy, y + dx), (dy, dx), path_cost, 1);
        self.enter((x - dy, y - dx), (-dy, -dx), path_cost, 1);

        if straight < 3 {
            self.enter((x + dx, y + dy), (dx, dy), path_cost, straight + 1);
        }
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut tiles: Array2<u8> = Array2::zeros((cols, rows));
    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            tiles[(x, y)] = ch.to_digit(10).unwrap() as u8;
        })
    });

    let mems = Array2::from_elem((cols, rows), HashMap::new());

    // I think it theoretically could be higher but isn't for this data-set
    let min_cost: usize = tiles.slice(s![.., ..]).iter().map(|&a| a as usize).sum();

    let mut board = Board { tiles, mems, min_cost};
    board.enter((1, 0), (1, 0), 0, 1);
    board.enter((0, 1), (0, 1), 0, 1);

    let result = board.min_cost;

    println!("{result}");
}
