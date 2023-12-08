use std::io;

const MAX_NODES: usize = 26*26*26;
const LAST_NODE: usize = MAX_NODES - 1;

#[derive(Clone)]
struct Node {
    pub left: u32,
    pub right: u32
}

fn node2int(s: &str) -> u32 {
    let bytes = s.as_bytes();

    bytes.iter().fold(0 as u32, |acc, &b| {
        acc * 26 + (b - 65) as u32
    })
}

fn main() {
    let mut lines = io::stdin().lines();
    let lr = lines.next().unwrap().unwrap();
    let lr: Vec<bool> = lr.chars().map(|ch| ch == 'R').collect();

    lines.next();

    let mut nodes: Vec<Option<Node>> = vec![None; MAX_NODES];

    lines.for_each(|line| {
        let line = line.unwrap();
        let node = node2int(&line[0..3]) as usize;
        let left = node2int(&line[7..10]);
        let right = node2int(&line[12..15]);

        nodes[node] = Some(Node{ left, right });
    });

    let mut pos: usize = 0;

    let steps = lr.into_iter().cycle().enumerate().find(|(_i, lr)| {
        let node = nodes[pos].as_ref().unwrap();
        pos = if *lr { node.right } else { node.left } as usize;
        pos == LAST_NODE
    }).unwrap().0 + 1;

    println!("Steps: {steps}");
}
