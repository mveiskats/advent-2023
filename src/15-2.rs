use std::io;
use std::iter;

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, ch| (acc + (ch as usize)) * 17 % 256)
}

fn main() {
    let mut boxes: Vec<Vec<(&str, usize)>> =
        iter::repeat(vec![]).take(256).collect();

    let input = io::stdin().lines().next().unwrap().unwrap();

    input.split(',').for_each(|s| {
        if let Some((label, focal)) = s.split_once('=') {
            let focal = focal.parse::<usize>().unwrap();
            let h = hash(&label);

            if let Some(i) = boxes[h].iter().position(|(l, _)| *l == label) {
                boxes[h][i] = (label, focal);
            } else {
                boxes[h].push((label, focal));
            }
        } else if let Some((label, _)) = s.split_once('-') {
            let h = hash(&label);
            boxes[h].retain(|(l, _)| *l != label);
        }
    });

    let result = boxes.into_iter().enumerate().map(|(i, v)|{
        v.into_iter().enumerate().map(|(j, (_, focal))| (i + 1) * (j + 1) * focal).sum::<usize>()
    }).sum::<usize>();

    println!("{result}");
}
