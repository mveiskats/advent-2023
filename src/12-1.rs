use std::io;

fn ways(cond: &[char], dam: &[usize]) -> usize {
    if cond.len() == 0 && dam.len() == 0 { return 1 }
    if cond.len() == 0 && dam.len() > 0 { return 0 }
    if dam.len() == 0 {
        return if cond.iter().any(|&ch| ch == '#') { 0 } else { 1 }
    }

    match cond[0] {
        '.' => ways(&cond[1..], &dam),
        '#' => dways(&cond, &dam),
        '?' => dways(&cond, &dam) + ways(&cond[1..], &dam),
        _ => panic!()
    }
}

fn dways(cond: &[char], dam: &[usize]) -> usize {
    if cond.len() < dam[0] { return 0 }
    if cond[0..dam[0]].iter().any(|&ch| ch == '.') { return 0 }
    if cond.len() > dam[0] {
        if cond[dam[0]] == '#' { return 0 }
        ways(&cond[(dam[0] + 1)..], &dam[1..])
    } else {
        ways(&cond[dam[0]..], &dam[1..])
    }
}

fn main() {
    let result: usize = io::stdin().lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (cond, dam) = line.split_once(' ').unwrap();
            let cond: Vec<char> = cond.chars().collect();
            let dam: Vec<usize> = dam.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

            ways(&cond, &dam)
        }).sum();

    println!("{result}");
}
