use std::io;

#[derive(Debug)]
struct MapEntry {
    pub in_start: usize,
    pub out_start: usize,
    pub len: usize
}

#[derive(Debug)]
struct Map {
    pub entries: Vec<MapEntry>
}

impl Map {
    fn translate(&self, n: usize) -> usize{
        if let Some(entry) = self.entries.iter().find(|e| e.in_start <= n && e.in_start + e.len > n) {
            n + entry.out_start - entry.in_start
        } else {
            n
        }
    }
}

fn main() {
    let mut lines = io::stdin().lines().map(|l| l.unwrap());

    let seeds = lines.next().unwrap();
    let seeds: Vec<usize> = seeds[7..].split(' ').map(|s| s.parse::<usize>().unwrap()).collect();
    lines.next();

    let mut maps: Vec<Map> = vec![];

    for _ in 0..7 {
        lines.next(); // Header
        maps.push(Map {
            entries: lines.by_ref()
                .take_while(|line| line.len() > 0)
                .map(|line| {
                    line.splitn(3, ' ')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .map(|arr| MapEntry { out_start: arr[0], in_start: arr[1], len: arr[2] })
                .collect()
        });
    }

    let closest = seeds.into_iter().map(|seed| {
        maps.iter().fold(seed, |n, map| map.translate(n))
    }).min().unwrap();

    println!("{closest}");
}
