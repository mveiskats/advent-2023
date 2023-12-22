use std::io;
use ndarray::{ s, Array2 };

//const STEP_TARGET: usize = 17;
const STEP_TARGET: usize = 26501365;

fn plot_count(walls: &Array2<bool>, distances: &Array2<usize>, depth: usize, oddity: usize) -> usize {
    let size = walls.dim().0;
    let mut plots = 0;
    for x in 0..size {
        for y in 0..size {
            if walls[(x, y)] { continue }
            if (x + y) % 2 == oddity && distances[(x, y)] <= depth {
                plots += 1;
            }
        }
    }
    plots
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut walls: Array2<bool> = Array2::from_elem((cols, rows), false);

    let mut start_x: usize = 0;
    let mut start_y: usize = 0;

    for (y, line) in lines.into_iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    start_x = x;
                    start_y = y;
                },
                '#' => walls[(x, y)] = true,
                _ => ()
            }
        }
    }

    let size = rows; // Input is square
    let half_size = start_x; // Distance from start to all sides is equal

    assert!(rows == cols);
    assert!(start_x == start_y);
    assert!(half_size * 2 + 1 == size);

    // Start row / col is empty
    assert!(walls.slice(s![start_x, ..]).iter().all(|wall| !wall), "start col not empty");
    assert!(walls.slice(s![.., start_y]).iter().all(|wall| !wall), "start row not empty");

    // Edges are empty
    assert!(walls.slice(s![0, ..]).iter().all(|wall| !wall), "left side not empty");
    assert!(walls.slice(s![-1, ..]).iter().all(|wall| !wall), "right side not empty");
    assert!(walls.slice(s![.., 0]).iter().all(|wall| !wall), "top side not empty");
    assert!(walls.slice(s![.., -1]).iter().all(|wall| !wall), "bottom side not empty");

    // All 3x3 possible entries in the tile.
    // We can be sure tiles will be entered via these first,
    // because start row and column as well as edges are conveniently empty.
    let entries: Vec<Vec<(usize, usize)>> = (0..3).map(|y| (0..3).map(|x| (x * half_size, y * half_size)).collect()).collect();

    // All of the distances in tile from each entry
    let distances: Vec<Vec<Array2<usize>>> = entries.iter().map(|entry_row| {
        entry_row.iter().map(|&(entry_x, entry_y)| {
            let mut distances = Array2::from_elem((size, size), usize::MAX);
            let mut visited: Array2<bool> = Array2::from_elem((size, size), false);
            let mut from: Vec<(usize, usize)> = vec![(entry_x, entry_y)];

            let mut current_distance = 0;
            while from.len() > 0 {
                let mut to: Vec<(usize, usize)> = vec![];
                for (x, y) in from {
                    if walls[(x, y)] { continue }
                    if visited[(x, y)] { continue }

                    visited[(x, y)] = true;
                    distances[(x, y)] = current_distance;

                    if x > 0 { to.push((x - 1, y)) }
                    if y > 0 { to.push((x, y - 1)) }
                    if x < cols - 1 { to.push((x + 1, y)) }
                    if y < rows - 1 { to.push((x, y + 1)) }
                }

                from = to;
                current_distance += 1;
            }

            distances
        }).collect()
    }).collect();

    assert!(STEP_TARGET > half_size + size, "didn't implement for small step values");

    // Should (x + y) be even or odd at the center sector
    let center_oddity = STEP_TARGET % 2;

    let mut plots = 0;

    // Furthest horizontal / vertical sector
    let side1_depth = (STEP_TARGET - half_size - 1) % size;
    let side1_oddity = if ((STEP_TARGET - half_size - 1) / size) % 2 == 0 { 1 - center_oddity } else { center_oddity };

    plots += plot_count(&walls, &distances[1][0], side1_depth, side1_oddity); // Enter from left
    plots += plot_count(&walls, &distances[1][2], side1_depth, side1_oddity); // Enter from right
    plots += plot_count(&walls, &distances[0][1], side1_depth, side1_oddity); // Enter from top
    plots += plot_count(&walls, &distances[2][1], side1_depth, side1_oddity); // Enter from bottom

    if side1_depth < half_size - 1 {
        // Second furthest horizontal / vertical sector
        let side2_depth = side1_depth + size;
        let side2_oddity = 1 - side1_oddity;

        plots += plot_count(&walls, &distances[1][0], side2_depth, side2_oddity); // Enter from left
        plots += plot_count(&walls, &distances[1][2], side2_depth, side2_oddity); // Enter from right
        plots += plot_count(&walls, &distances[0][1], side2_depth, side2_oddity); // Enter from top
        plots += plot_count(&walls, &distances[2][1], side2_depth, side2_oddity); // Enter from bottom

    }

    // Diagonal sectors beginning next to furthest straight sector
    if side1_depth > half_size {
        let diag1_depth = side1_depth - half_size - 1;
        let diag1_sectors = (STEP_TARGET - 1) / size;
        let diag1_oddity = 1 - side1_oddity;

        plots += diag1_sectors * plot_count(&walls, &distances[0][0], diag1_depth, diag1_oddity); // Enter from left
        plots += diag1_sectors * plot_count(&walls, &distances[0][2], diag1_depth, diag1_oddity); // Enter from right
        plots += diag1_sectors * plot_count(&walls, &distances[2][0], diag1_depth, diag1_oddity); // Enter from top
        plots += diag1_sectors * plot_count(&walls, &distances[2][2], diag1_depth, diag1_oddity); // Enter from bottom
    }

    // Diagonal sectors beginning next to second furthest straight sector
    let diag2_depth = side1_depth + size - half_size - 1;
    let diag2_sectors = (STEP_TARGET - half_size - 1) / size;
    let diag2_oddity = side1_oddity;

    plots += diag2_sectors * plot_count(&walls, &distances[0][0], diag2_depth, diag2_oddity); // Enter from left
    plots += diag2_sectors * plot_count(&walls, &distances[0][2], diag2_depth, diag2_oddity); // Enter from right
    plots += diag2_sectors * plot_count(&walls, &distances[2][0], diag2_depth, diag2_oddity); // Enter from top
    plots += diag2_sectors * plot_count(&walls, &distances[2][2], diag2_depth, diag2_oddity); // Enter from bottom

    if side1_depth == 0 {
        // Diagonal sectors beginning next to third furthest straight sector
        let diag3_depth = size + half_size;
        let diag3_sectors = diag2_sectors - 1;
        let diag3_oddity = 1 - diag2_oddity;

        plots += diag3_sectors * plot_count(&walls, &distances[0][0], diag3_depth, diag3_oddity); // Enter from left
        plots += diag3_sectors * plot_count(&walls, &distances[0][2], diag3_depth, diag3_oddity); // Enter from right
        plots += diag3_sectors * plot_count(&walls, &distances[2][0], diag3_depth, diag3_oddity); // Enter from top
        plots += diag3_sectors * plot_count(&walls, &distances[2][2], diag3_depth, diag3_oddity); // Enter from bottom
    }

    // Fully filled sectors
    let full_straight_sectors: usize = (STEP_TARGET - size) / size;
    let full_quadrant_sectors: usize = 4 * (0..=full_straight_sectors).filter(|i| i % 2 == 0).sum::<usize>() + 1;
    let full_inverted_quadrant_sectors: usize = 4 * (0..=full_straight_sectors).filter(|i| i % 2 == 1).sum::<usize>();

    let mut full_sector_plots = 0;
    let mut full_inverted_sector_plots = 0;

    for x in 0..size {
        for y in 0..size {
            if walls[(x, y)] { continue }
            if distances[1][1][(x, y)] == usize::MAX { continue }
            if (x + y) % 2 == center_oddity {
                full_sector_plots += 1;
            } else {
                full_inverted_sector_plots += 1;
            }
        }
    }

    let plots = plots +
        (full_quadrant_sectors * full_sector_plots) +
        (full_inverted_quadrant_sectors * full_inverted_sector_plots);

    println!("{plots}");
}
