use std::io;

fn main() {
    let values = [
        (1, "1"), (2, "2"), (3, "3"), (4, "4"), (5, "5"), (6, "6"), (7, "7"), (8, "8"), (9, "9"),
        (1, "one"), (2,"two"), (3, "three"), (4, "four"), (5, "five"), (6, "six"), (7, "seven"), (8, "eight"), (9, "nine")
    ];

    let lines: Vec<String> = io::stdin().lines().map(|line| line.unwrap()).collect();

    let firsts = lines.iter().map(|line| {
        values.iter()
            .flat_map(|(val, s)| if let Some(pos) = line.find(s) { Some((*val, pos)) } else { None })
            .min_by_key(|(_, pos)| *pos).unwrap().0
    });
    let lasts = lines.iter().map(|line| {
        values.iter()
            .flat_map(|(val, s)| if let Some(pos) = line.rfind(s) { Some((*val, pos + s.len())) } else { None })
            .max_by_key(|(_, pos)| *pos).unwrap().0
    });

    let total: usize = firsts.zip(lasts).map(|(f, l)| f * 10 + l).sum();

    println!("{total}");
}
