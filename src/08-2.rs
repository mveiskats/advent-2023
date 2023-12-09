use std::io;

const RADIX: usize = 26;
const MAX_NODES: usize = RADIX * RADIX * RADIX;

#[derive(Clone)]
struct Node {
    pub left: usize,
    pub right: usize
}

#[derive(Debug)]
struct Path {
    pub start_node: usize,
    pub cycle_start: usize,
    pub cycle_length: usize,
    pub exit_offset: usize
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn node2int(s: &str) -> usize {
    let bytes = s.as_bytes();

    bytes.iter().fold(0 as usize, |acc, &b| {
        acc * RADIX + (b - 65) as usize
    })
}

fn main() {
    let mut lines = io::stdin().lines();
    let directions = lines.next().unwrap().unwrap();
    let directions: Vec<bool> = directions.chars().map(|ch| ch == 'R').collect();

    lines.next();

    let mut nodes: Vec<Option<Node>> = vec![None; MAX_NODES];
    let mut starts: Vec<usize> = vec![];
    //let mut ends: Vec<usize> = vec![];

    lines.for_each(|line| {
        let line = line.unwrap();
        let node = node2int(&line[0..3]);
        let left = node2int(&line[7..10]);
        let right = node2int(&line[12..15]);

        // Start with nodes that end with A
        if node % RADIX == 0 { starts.push(node) }

        nodes[node] = Some(Node{ left, right });
    });

    let paths: Vec<Path> = starts.iter().map(|start| {
        let mut cursor = *start;
        let mut visit_step: Vec<Option<usize>> = vec![None; MAX_NODES];
        // There's only one end node on each path cycle
        let mut exit_idx: usize = 0;

        let dir_len = directions.len();
        directions.iter().cycle().enumerate().find_map(|(i, lr)| {
            let node = nodes[cursor].as_ref().unwrap();
            cursor = if *lr { node.right } else { node.left };

            if exit_idx == 0 && cursor % RADIX == RADIX - 1 { exit_idx = i }

            if let Some(n) = visit_step[cursor] {
                if (i - n) % dir_len == 0 {
                    Some(Path { start_node: *start, cycle_start: n, cycle_length: i - n, exit_offset: exit_idx - n })
                } else {
                    None
                }
            } else {
                visit_step[cursor] = Some(i);
                None
            }
        }).unwrap()
    }).collect();

    paths.iter().for_each(|path| {
        println!("start_node: {}, cycle_start: {}, cycle_length: {}, exit_offset: {}, delta: {}",
                 path.start_node, path.cycle_start, path.cycle_length, path.exit_offset, path.cycle_length - path.exit_offset);
    });

    let cycle_lcm = paths.iter().fold(1, |acc, path| lcm(acc, path.cycle_length));

    println!("{}", cycle_lcm);
}
