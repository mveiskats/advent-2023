use std::io;

struct Board {
    pub cells: Vec<Vec<char>>,
    pub rows: usize,
    pub cols: usize,
    pub nodes: usize,
    pub adjacency: Vec<Vec<bool>>,
    pub start: (usize, usize)
}

impl Board {
    fn parse() -> Self {
        let cells: Vec<Vec<char>> = io::stdin().lines().map(|line| line.unwrap().chars().collect()).collect();

        let rows = cells.len();
        let cols = cells[0].len();
        let nodes = rows * cols;

        Self {
            cells, rows, cols, nodes,
            adjacency: vec![vec![false; nodes]; nodes],
            start: (0, 0)
        }
    }

    fn cell_idx(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn set_adjacent(&mut self, row_a: usize, col_a: usize, row_b: isize, col_b: isize) {
        if col_b < 0 || col_b >= self.cols as isize { return }
        if row_b < 0 || row_b >= self.rows as isize { return }

        let cell_a = self.cell_idx(row_a, col_a);
        let cell_b = self.cell_idx(row_b as usize, col_b as usize);

        self.adjacency[cell_a][cell_b] = true;
    }

    fn is_adjacent(&self, row_a: usize, col_a: usize, row_b: usize, col_b: usize) -> bool {
        let cell_a = self.cell_idx(row_a, col_a);
        let cell_b = self.cell_idx(row_b, col_b);

        self.adjacency[cell_a][cell_b]
    }

    fn build_adjacency(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let ir = r as isize;
                let ic = c as isize;

                match self.cells[r][c] {
                    'S' => {
                        self.start = (r, c);

                        self.set_adjacent(r, c, ir - 1, ic);
                        self.set_adjacent(r, c, ir + 1, ic);
                        self.set_adjacent(r, c, ir, ic - 1);
                        self.set_adjacent(r, c, ir, ic + 1);
                    },
                    '|' => {
                        self.set_adjacent(r, c, ir - 1, ic);
                        self.set_adjacent(r, c, ir + 1, ic);
                    },
                    '-' => {
                        self.set_adjacent(r, c, ir, ic - 1);
                        self.set_adjacent(r, c, ir, ic + 1);
                    },
                    'L' => {
                        self.set_adjacent(r, c, ir - 1, ic);
                        self.set_adjacent(r, c, ir, ic + 1);
                    },
                    'J' => {
                        self.set_adjacent(r, c, ir - 1, ic);
                        self.set_adjacent(r, c, ir, ic - 1);
                    },
                    '7' => {
                        self.set_adjacent(r, c, ir + 1, ic);
                        self.set_adjacent(r, c, ir, ic - 1);
                    },
                    'F' => {
                        self.set_adjacent(r, c, ir + 1, ic);
                        self.set_adjacent(r, c, ir, ic + 1);
                    },
                    _ => ()
                }
            }
        }
    }

    fn clean_connections(&mut self) {
        for i in 0..self.nodes {
            for j in 0..self.nodes {
                if i == j { continue }
                if self.adjacency[i][j] && !self.adjacency[j][i] { self.adjacency[i][j] = false }
            }
        }
    }

    fn find_enclosed(&self) -> usize {
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.cols]; self.rows];

        let mut front: Vec<(usize, usize)> = vec![self.start];

        while front.len() > 0 {
            let mut next_front: Vec<(usize, usize)> = vec![];
            for (r, c) in &front {
                visited[*r][*c] = true;
                let idx_a = self.cell_idx(*r, *c);
                for idx_b in 0..self.nodes {
                    if self.adjacency[idx_a][idx_b] {
                        let row_b = idx_b / self.cols;
                        let col_b = idx_b % self.cols;

                        if !visited[row_b][col_b] {
                            next_front.push((row_b, col_b));
                        }
                    }
                }
            }
            front = next_front;
        }

        let mut ext_border: Vec<Vec<bool>> = vec![vec![false; self.cols * 2]; self.rows * 2];
        for r in 0..self.rows {
            for c in 0..self.cols {
                ext_border[r * 2][c * 2] = visited[r][c];
            }
        }

        for r in 0..self.rows {
            for c in 0..self.cols {
                if !visited[r][c] { continue }
                if r < self.rows - 1 && self.is_adjacent(r, c, r + 1, c) { ext_border[r * 2 + 1][c * 2] = true }
                if c < self.cols - 1 && self.is_adjacent(r, c, r, c + 1) { ext_border[r * 2][c * 2 + 1] = true }
            }
        }

        [(self.start.0 * 2 - 1, self.start.1 * 2 - 1),
         (self.start.0 * 2 - 1, self.start.1 * 2 + 1),
         (self.start.0 * 2 + 1, self.start.1 * 2 - 1),
         (self.start.0 * 2 + 1, self.start.1 * 2 + 1)].iter().find_map(|(start_rx, start_cx)| {
             let mut ext_painted: Vec<Vec<bool>> = vec![vec![false; self.cols * 2]; self.rows * 2];
             let mut front: Vec<(usize, usize)> = vec![(*start_rx, *start_cx)];
             let mut area = 0;
             let mut inside = true;

             while front.len() > 0 && inside {
                 let mut next_front: Vec<(usize, usize)> = vec![];

                 for (rx, cx) in &front {
                     let rx = *rx;
                     let cx = *cx;

                     if ext_border[rx][cx] { continue }
                     if ext_painted[rx][cx] { continue }

                     if rx == 0 || rx == self.rows * 2 - 1 || cx == 0 || cx == self.cols * 2 - 1 {
                         inside = false;
                         continue;
                     }

                     if rx % 2 == 0 && cx % 2 == 0 { area += 1 }
                     ext_painted[rx][cx] = true;

                     next_front.push((rx - 1, cx));
                     next_front.push((rx + 1, cx));
                     next_front.push((rx, cx - 1));
                     next_front.push((rx, cx + 1));
                 }

                 front = next_front;
             }

             if inside { Some(area) } else { None }
         }).unwrap()
    }
}

fn main() {
    let mut board = Board::parse();

    board.build_adjacency();
    board.clean_connections();

    println!("{}", board.find_enclosed());
}
