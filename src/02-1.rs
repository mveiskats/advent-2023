use std::io;

fn main() {
    let result: usize = io::stdin().lines().filter_map(|line| {
        let line = line.expect("read error");
        let (game_id, cubes) = line.split_once(": ").unwrap();
        let game_id = game_id[5..].parse::<usize>().unwrap();

        if cubes.split("; ").all(|handful| {
            handful.split(", ").all(|cube| {
                let (number, color) = cube.split_once(' ').unwrap();
                let number = number.parse::<usize>().unwrap();
                (color == "red" && number <= 12) ||
                    (color == "green" && number <= 13) ||
                    (color == "blue" && number <= 14)
            })
        }) {
            Some(game_id)
        } else {
            None
        }
    }).sum();

    println!("{result}");
}
