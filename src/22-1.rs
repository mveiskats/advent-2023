use std::io;
use ndarray::{ s, Array3 };

type Point = [usize; 3];

#[derive(Debug)]
struct Brick {
    start: Point,
    end: Point
}

impl Brick {
    fn colliders(&self) -> Vec<Point> {
        if self.start[0] < self.end[0] {
            self.points()
        } else if self.start[1] < self.end[1] {
            self.points()
        } else {
            // For vertical bricks only the start is colliding
            vec![self.start]
        }
    }

    fn points(&self) -> Vec<Point> {
        if self.start[0] < self.end[0] {
            (self.start[0]..=self.end[0]).map(|i| [i, self.start[1], self.start[2]]).collect()
        } else if self.start[1] < self.end[1] {
            (self.start[1]..=self.end[1]).map(|i| [self.start[0], i, self.start[2]]).collect()
        } else {
            (self.start[2]..=self.end[2]).map(|i| [self.start[0], self.start[1], i]).collect()
        }
    }

    fn min_z(&self) -> usize {
        self.points().into_iter().map(|p| p[2]).min().unwrap()
    }
}

fn main() {
    let mut bricks: Vec<Brick> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            let start = start.split(',').map(str::parse::<usize>).map(Result::unwrap).collect::<Vec<_>>();
            let start = [start[0], start[1], start[2]];
            let end = end.split(',').map(str::parse::<usize>).map(Result::unwrap).collect::<Vec<_>>();
            let end = [end[0], end[1], end[2]];

            assert!(start.iter().zip(end.iter()).filter(|(s, e)| e > s).count() <= 1, "multi-dimensional brick");

            Brick { start, end }
        })
        .collect();

    bricks.sort_by_key(Brick::min_z);

    let x_size = 1 + bricks.iter().map(|brick| brick.end[0]).max().expect("no bricks");
    let y_size = 1 + bricks.iter().map(|brick| brick.end[1]).max().expect("no bricks");
    let z_size = 1 + bricks.iter().map(|brick| brick.end[2]).max().expect("no bricks");

    let mut resting: Array3<Option<usize>> = Array3::from_elem((x_size, y_size, z_size), None);
    let mut supported_by: Vec<Vec<usize>> = vec![vec![]; bricks.len()];

    for (i, brick) in bricks.iter().enumerate() {
        let colliders = brick.colliders();
        let points = brick.points();

        let fall_room: usize = colliders.iter().map(|p| {
            resting.slice(s![p[0], p[1], 1..p[2];-1]).iter()
                .take_while(|&&spot| spot == None)
                .count()
        }).min().unwrap();

        for point in points.iter() {
            let new_point = (point[0], point[1], point[2] - fall_room);
            if resting[new_point] != None {
                panic!("brick {i} fell into non-empty position");
            }
            resting[new_point] = Some(i);
        }

        supported_by[i] = colliders.into_iter()
            .map(|p| resting[(p[0], p[1], p[2] - fall_room - 1)] ) // Cell below collider
            .flatten() // Remove None
            .collect();

        // Remove duplicates
        supported_by[i].sort();
        supported_by[i].dedup();
    }

    let result: usize = (0..bricks.len())
        .filter(|&i| !supported_by.iter().any(|sup| *sup == [i]))
        .count();

    println!("{result}");
}
