use std::io;
use std::cmp;

fn main() {
    let result: usize = io::stdin().lines().map(|line| {
        let line = line.expect("read error");
        let (_game_id, cubes) = line.split_once(": ").unwrap();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        cubes.split("; ").for_each(|handful| {
            handful.split(", ").for_each(|cube| {
                let (number, color) = cube.split_once(' ').unwrap();
                let number = number.parse::<usize>().unwrap();

                match color {
                    "red" => min_red = cmp::max(min_red, number),
                    "green" => min_green = cmp::max(min_green, number),
                    "blue" => min_blue = cmp::max(min_blue, number),
                    _ => panic!()
                }
            })
        });

        min_red * min_green * min_blue
    }).sum();

    println!("{result}");
}
