use std::io;
use ndarray::Array2;

fn main() {
    let input: Vec<(char, usize)> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| {
            let mut parts = line.split(' ');
            let dir = parts.next().unwrap().chars().next().unwrap();
            let len = parts.next().unwrap().parse::<usize>().unwrap();
            (dir, len)
        })
        .collect();

    let mut minx: isize = 0;
    let mut miny: isize = 0;
    let mut maxx: isize = 0;
    let mut maxy: isize = 0;

    let mut x: isize = 0;
    let mut y: isize = 0;

    // Determine bounds
    input.iter().for_each(|&(ch, len)| {
        match ch {
            'U' => y -= len as isize,
            'D' => y += len as isize,
            'L' => x -= len as isize,
            'R' => x += len as isize,
            _ => panic!("unknown direction")
        }
        if x < minx { minx = x }
        if x > maxx { maxx = x }
        if y < miny { miny = y }
        if y > maxy { maxy = y }
    });

    // Add 1 row of padding around the area
    let rows = (maxy - miny + 1 + 2) as usize;
    let cols = (maxx - minx + 1 + 2) as usize;
    let mut x = (-minx + 1) as usize;
    let mut y = (-miny + 1) as usize;

    let mut edges: Array2<bool> = Array2::from_elem((cols, rows), false);
    let mut edge_count = 0;

    input.iter().for_each(|&(ch, len)| {
        let (dx, dy): (isize, isize) = match ch {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("unknown direction")
        };

        for _ in 0..len {
            x = (x as isize + dx) as usize;
            y = (y as isize + dy) as usize;

            edges[(x, y)] = true;
            edge_count += 1;
        }
    });

    let corners = [
        (x - 1, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y + 1)
    ];

    // Flood fill from each corner and see if it reaches the edge
    let result: usize = corners.into_iter().map(|(startx, starty)| {
        let mut visited = edges.clone();
        let mut interior_count = 0;

        let mut stack: Vec<(usize, usize)> = vec![(startx, starty)];
        while stack.len() > 0 {
            let (x, y) = stack.pop().unwrap();
            if visited[(x, y)] { continue }

            // Touched an edge - not an interior
            if x == 0 || y == 0 || x == cols - 1 || y == rows - 1 { return None }

            visited[(x, y)] = true;
            interior_count += 1;

            if x > 0 { stack.push((x - 1, y)) }
            if y > 0 { stack.push((x, y - 1)) }
            if x < cols - 1 { stack.push((x + 1, y)) }
            if y < rows - 1 { stack.push((x, y + 1)) }
        }
        Some(edge_count + interior_count)
    }).flatten().next().unwrap();

    println!("{:?}", result);
}
