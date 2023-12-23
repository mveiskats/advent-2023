use std::io;
use ndarray::{ s, Array2 };

#[derive(Debug, Clone)]
struct Path {
    start: (usize, usize),
    end: (usize, usize),
    len: usize,
}

struct Map {
    tiles: Array2<char>,
    visited: Array2<bool>,
    paths: Vec<Path>,
    junctions: Vec<(usize, usize)>,
    nodes: Vec<Path>,
    nodes_adjacent: Vec<Vec<usize>>,
}

impl Map {
    fn from(tiles: Array2<char>) -> Self {
        let (cols, rows) = tiles.dim();

        Self {
            tiles,
            visited: Array2::from_elem((cols, rows), false),
            paths: vec![],
            junctions: vec![],
            nodes: vec![],
            nodes_adjacent: vec![],
        }
    }

    fn adjacent((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> bool {
        (x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs() == 1
    }

    // Will panic on edge tiles
    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
            .filter(|&(nx, ny)| self.tiles[(nx, ny)] != '#')
            .collect()
    }

    fn unvisited_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        self.neighbors(x, y).into_iter()
            .filter(|&(nx, ny)| !self.visited[(nx, ny)])
            .collect()
    }

    fn is_junction(&self, x: usize, y: usize) -> bool {
        self.neighbors(x, y).len() > 2
    }

    fn visit_path(&mut self, (start_x, start_y): (usize, usize), (prev_x, prev_y): (usize, usize), (x, y): (usize, usize), path_len: usize) {
        let (_cols, rows) = self.tiles.dim();
        // Reached the end
        if y == rows - 1 {
            self.paths.push(Path { start: (start_x, start_y), end: (x, y), len: path_len + 1});
            self.visited[(x, y)] = true;
            return
        }

        if self.is_junction(x, y) {
            self.paths.push(Path { start: (start_x, start_y), end: (prev_x, prev_y), len: path_len });
            self.visit_junction(x, y)
        } else {
            self.visited[(x, y)] = true;

            if let Some(&(next_x, next_y)) = self.unvisited_neighbors(x, y).first() {
                self.visit_path((start_x, start_y), (x, y), (next_x, next_y), path_len + 1);
            } else {
                self.paths.push(Path { start: (start_x, start_y), end: (x, y), len: path_len + 1 });
            }
        }
    }

    fn visit_junction(&mut self, x: usize, y: usize) {
        if self.visited[(x, y)] { return }

        self.visited[(x, y)] = true;
        self.junctions.push((x, y));

        while let Some((nx, ny)) = self.unvisited_neighbors(x, y).into_iter().next() {
            self.visit_path((nx, ny), (nx, ny), (nx, ny), 0);
        }
    }

    fn max_len(&self, node: usize, route_len: usize, route: &Vec<usize>) -> usize {
        let route_len = route_len + self.nodes[node].len;
        if node == 1 {
            // Reached end node
            return route_len;
        } else if route.contains(&node) {
            return 0;
        } else {
            let route = route.iter().copied().chain([node].into_iter()).collect();

            self.nodes_adjacent[node].iter()
                .map(|&next| self.max_len(next, route_len, &route))
                .max()
                .unwrap()
        }
    }

    fn longest_hike(&mut self) -> usize {
        let (_cols, rows) = self.tiles.dim();

        let start_x = self.tiles.slice(s![.., 0]).iter().position(|&ch| ch == '.').expect("no start");
        let end_x = self.tiles.slice(s![.., -1]).iter().position(|&ch| ch == '.').expect("no end");
        let start = (start_x, 0);
        let end = (end_x, rows - 1);

        self.visited[start] = true;
        self.visited[end] = true;
        self.junctions.push(start);
        self.junctions.push(end);

        self.visit_path((start_x, 1), (start_x, 1), (start_x, 1), 0);

        // Convert junctions to 1-long paths
        for &(x, y) in self.junctions.iter() {
            self.nodes.push(Path { start: (x, y), end: (x, y), len: 1 });
            self.nodes_adjacent.push(vec![]);
        }

        for path in self.paths.iter() {
            self.nodes.push(path.clone());
            self.nodes_adjacent.push(vec![]);

            let current_node = self.nodes.len() - 1;

            // Build adjacency lists
            for j in 0..self.junctions.len() {
                if Self::adjacent(path.start, self.junctions[j]) || Self::adjacent(path.end, self.junctions[j]) {
                    self.nodes_adjacent[j].push(current_node);
                    self.nodes_adjacent[current_node].push(j);
                }
            }
        }

        self.max_len(0, 0, &vec![]) - 1
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

    let mut map = Map::from(tiles);

    println!("{}", map.longest_hike());
}
