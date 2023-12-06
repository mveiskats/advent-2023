use std::env;

fn main() {
    // CBA to write parsing logic - just input the values into args
    let args: Vec<String> = env::args().collect();
    let time = args[1].parse::<usize>().unwrap();
    let record = args[2].parse::<usize>().unwrap();

    let wins = (1..time)
        .filter(|charge| charge * (time - charge) > record)
        .count();

    println!("{wins} ways to win");
}
