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

        println!("Nodes: {nodes}");

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

    fn find_furthest(&self) -> usize {
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.cols]; self.rows];

        let mut front: Vec<(usize, usize)> = vec![self.start];

        (0..).find(|_i| {
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
            front.len() == 0
        }).unwrap()
    }
}

fn main() {
    let mut board = Board::parse();

    board.build_adjacency();
    board.clean_connections();

    println!("{}", board.find_furthest());
}
