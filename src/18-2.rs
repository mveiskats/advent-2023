use std::io;
use std::cmp;
use ndarray::Array2;

type Side = (isize, isize, isize);

struct Board {
    start: (isize, isize),
    h_sides: Vec<Side>,
    v_sides: Vec<Side>,
    col_widths: Vec<isize>,
    col_offsets: Vec<isize>,
    row_widths: Vec<isize>,
    row_offsets: Vec<isize>
}

impl Board {
    fn from(start: (isize, isize), h_sides: Vec<Side>, v_sides: Vec<Side>) -> Self {
        // Side endpoints = row/col dividers
        let mut x_dividers: Vec<isize> = v_sides.iter().map(|(x, _, _)| *x).collect();
        x_dividers.dedup();

        let mut y_dividers: Vec<isize> = h_sides.iter().map(|(y, _, _)| *y).collect();
        y_dividers.dedup();

        // Each divider gets separate row/col with width 1
        let mut col_widths: Vec<isize> = vec![0];
        for i in 0..(x_dividers.len() - 1) {
            let diff = x_dividers[i + 1] - x_dividers[i];
            col_widths.push(1);
            col_widths.push(diff - 1);
        }
        col_widths.push(1);
        col_widths.push(0);

        let mut row_widths: Vec<isize> = vec![0];
        for i in 0..(y_dividers.len() - 1) {
            let diff = y_dividers[i + 1] - y_dividers[i];
            row_widths.push(1);
            row_widths.push(diff - 1);
        }
        row_widths.push(1);
        row_widths.push(0);


        let rows = row_widths.len();
        let cols = col_widths.len();

        let mut col_offsets: Vec<isize> = vec![0; cols];
        for i in 0..(cols - 1) {
            col_offsets[i + 1] = col_offsets[i] + col_widths[i];
        }

        let mut row_offsets: Vec<isize> = vec![0; rows];
        for i in 0..(rows - 1) {
            row_offsets[i + 1] = row_offsets[i] + row_widths[i];
        }

        Self { start, h_sides, v_sides, col_widths, col_offsets, row_widths, row_offsets }
    }

    fn cols(&self) -> usize {
        self.col_widths.len()
    }

    fn rows(&self) -> usize {
        self.row_widths.len()
    }

    fn x2col(&self, x: isize) -> usize {
        (0..self.cols()).take_while(|&i| self.col_offsets[i] <= x).last().unwrap()
    }

    fn y2row(&self, y: isize) -> usize {
        (0..self.rows()).take_while(|&i| self.row_offsets[i] <= y).last().unwrap()
    }

    fn area(&self) -> isize {
        let rows = self.rows();
        let cols = self.cols();
        let mut edges: Array2<bool> = Array2::from_elem((cols, rows), false);
        let mut edge_area: isize = 0;

        for &(y, start, end) in self.h_sides.iter() {
            let row = self.y2row(y);
            let start_col = self.x2col(start);
            let end_col = self.x2col(end);

            for col in start_col..=end_col {
                edges[(col, row)] = true;
            }

            edge_area += end - start;
        }

        for &(x, start, end) in self.v_sides.iter() {
            let col = self.x2col(x);
            let start_row = self.y2row(start);
            let end_row = self.y2row(end);

            for row in start_row..=end_row {
                edges[(col, row)] = true;
            }

            edge_area += end - start;
        }

        let (start_x, start_y) = self.start;
        let start_col = self.x2col(start_x);
        let start_row = self.y2row(start_y);

        let corners = [
            (start_col - 1, start_row - 1),
            (start_col + 1, start_row - 1),
            (start_col - 1, start_row + 1),
            (start_col + 1, start_row + 1)
        ];

        // Flood fill from each corner and see if it reaches the edge
        corners.into_iter().map(|(start_col, start_row)| {
            let mut visited = edges.clone();
            let mut interior_area = 0;

            let mut stack: Vec<(usize, usize)> = vec![(start_col, start_row)];
            while stack.len() > 0 {
                let (col, row) = stack.pop().unwrap();
                if visited[(col, row)] { continue }

                // Touched an edge - not an interior
                if col == 0 || row == 0 || col == cols - 1 || row == rows - 1 { return None }

                visited[(col, row)] = true;
                interior_area += self.col_widths[col] * self.row_widths[row];

                if col > 0 { stack.push((col - 1, row)) }
                if row > 0 { stack.push((col, row - 1)) }
                if col < cols - 1 { stack.push((col + 1, row)) }
                if row < rows - 1 { stack.push((col, row + 1)) }
            }
            Some(edge_area + interior_area)
        }).flatten().next().unwrap()
    }
}

fn main() {
    let input: Vec<(u8, isize)> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| {
            let code = line.split(' ').last().unwrap();
            let len = isize::from_str_radix(&code[2..7], 16).expect("invalid len");
            let dir = u8::from_str_radix(&code[7..8], 4).expect("invalid dir");
            (dir, len)
        })
        .collect();

    let mut minx: isize = 0;
    let mut miny: isize = 0;

    let mut x: isize = 0;
    let mut y: isize = 0;

    let mut h_sides: Vec<Side> = vec![];
    let mut v_sides: Vec<Side> = vec![];

    input.into_iter().for_each(|(dir, len)| {
        match dir {
            // R
            0 => {
                h_sides.push((y, x, x + len));
                x += len as isize;
            },
            // D
            1 => {
                v_sides.push((x, y, y + len));
                y += len as isize;
            },
            // L
            2 => {
                h_sides.push((y, x - len, x));
                x -= len as isize;
            },
            // U
            3 => {
                v_sides.push((x, y - len, y));
                y -= len as isize;
            },
            _ => panic!("unknown direction")
        }
        minx = cmp::min(x, minx);
        miny = cmp::min(y, miny);
    });

    h_sides = h_sides.into_iter().map(|(y, start, end)| (y - miny, start - minx, end - minx)).collect();
    v_sides = v_sides.into_iter().map(|(x, start, end)| (x - minx, start - miny, end - miny)).collect();

    h_sides.sort_by_key(|(y, _, _)| *y);
    v_sides.sort_by_key(|(x, _, _)| *x);

    let board = Board::from((-minx, -miny), h_sides, v_sides);

    println!("{}", board.area());
}
