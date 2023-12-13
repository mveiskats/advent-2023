use std::io;
use std::cmp;

const DEBUG_PRINT: bool = false;

#[derive(Copy, Clone, Debug, PartialEq)]
enum C { Broken, Maybe, Fine }

#[derive(Copy, Clone, Debug)]
struct Chunk {
    kind: C,
    start: usize,
    len: usize,
}

fn debug_print(msg: String, pad: usize) {
    if !DEBUG_PRINT { return }
    let padding = std::iter::repeat(' ').take(pad).collect::<String>();
    println!("{padding}{msg}");
}

fn fac(min: u128, max: u128) -> u128 {
    (min..=max).fold(1, |acc, x| acc * x)
}

fn c(n: usize, k: usize) -> usize {
    let n = n as u128;
    let k = k as u128;
    (fac(n - k + 1, n) / fac(1, k)) as usize
}

fn unrestricted_ways(excess_room: usize, group_count: usize, pad: usize) -> usize {
    let result = c(excess_room + group_count, group_count);
    debug_print(format!("unrestricted {excess_room}, {group_count} = {result}"), pad);
    result
}

fn fit_count(room: usize, groups: &[usize]) -> usize {
    let mut len: usize = 0;
    groups.iter().enumerate().take_while(|(i, g)| { len += *g; len + i <= room }).count()
}

fn fit_len(groups: &[usize]) -> usize {
    if groups.len() == 0 { return 0 }
    groups.iter().sum::<usize>() + groups.len() - 1
}

fn rewrite_chunks(len: usize, chunks: &[Chunk], pad: usize) -> Vec<Chunk> {
    debug_print(format!("rewrite_chunks {}, {:?}", len, chunks), pad);
    let mut l = len;
    let mut i = 0;
    while chunks[i].kind != C::Fine && l >= chunks[i].len {
        l -= chunks[i].len;
        i += 1;
    }

    let mut result: Vec<Chunk> = vec![];
    if chunks[i].kind == C::Fine {
        result.push(chunks[i].clone());
    } else {
        result.push(Chunk { kind: chunks[i].kind, len: &chunks[i].len - l, start: &chunks[i].start + l});
    }
    result.extend_from_slice(&chunks[(i + 1)..]);

    result
}

fn ways(pre_room: usize, chunks: &[Chunk], groups: &[usize], pad: usize) -> usize{
    debug_print(format!("ways {}, {:?}, {:?}", pre_room, chunks, groups), pad);
    let pad = pad + 2;

    if chunks.len() == 0 && groups.len() > 0 { return 0 }
    if chunks.len() == 0 && groups.len() == 0 { debug_print(String::from("Matched"), pad); return 1 }

    let chunk = &chunks[0];

    match chunk.kind {
        C::Fine => ways(0, &chunks[1..], &groups, pad),
        C::Maybe => {
            match chunks[1].kind {
                C::Broken => ways(chunk.len, &chunks[1..], &groups, pad),
                C::Fine => {
                    let max_fit = fit_count(chunk.len, &groups);

                    (0..=max_fit).map(|group_idx| {
                        let pre_ways = if group_idx > 0 {
                            let excess = chunk.len - fit_len(&groups[..group_idx]);
                            unrestricted_ways(excess, group_idx, pad)
                        } else {
                            1
                        };

                        pre_ways * ways(0, &chunks[1..], &groups[group_idx..], pad)
                    }).sum()
                },
                _ => panic!()
            }
        },
        C::Broken => {
            if groups.len() == 0 { return 0 }
            // Reserve 1 space for the gap before group matching the broken chunk
            let fit_room = if pre_room > 0 { pre_room - 1} else { 0 };
            let max_fit = fit_count(fit_room, &groups[..&groups.len() - 1]);

            let following_len = chunks[1..].iter()
                .take_while(|c| c.kind != C::Fine)
                .map(|c| c.len)
                .sum();

            (0..=max_fit).map(|group_idx| {
                debug_print(format!("fit {} groups before {}", group_idx, chunk.start), pad);
                let group_len = groups[group_idx];

                // Group can't be smaller than the chunk
                if group_len < chunk.len { return 0 }

                let max_ofs = group_len - chunk.len;
                let max_post_ofs = cmp::min(max_ofs, following_len);
                let max_pre_ofs = cmp::min(max_ofs, pre_room);
                let min_pre_ofs = max_ofs - max_post_ofs;

                (min_pre_ofs..=max_pre_ofs).map(|pre_ofs| {
                    debug_print(format!("check offset {pre_ofs}"), pad);

                    let pre_ways = if group_idx > 0 {
                        let group_fit = fit_len(&groups[..group_idx]);

                        if group_fit + pre_ofs > fit_room { return 0 }
                        let excess = fit_room - group_fit - pre_ofs;
                        unrestricted_ways(excess, group_idx, pad)
                    } else {
                        1
                    };

                    let post_ofs = max_ofs - pre_ofs;
                    // Chunk cannot end inside or touching another broken chunk
                    let group_end = chunk.start + chunk.len + post_ofs;
                    if chunks[1..].iter().any(|c| c.kind == C::Broken && group_end >= c.start && group_end < c.start + c.len) { return 0 }

                    // Remove 1 space for the gap after group matching the broken chunk
                    let chunks = rewrite_chunks(post_ofs + 1, &chunks[1..], pad);

                    pre_ways * ways(0, &chunks[..], &groups[(group_idx + 1)..], pad)
                }).sum()
            }).sum()
        }
    }
}


fn main() {
    let result: usize = io::stdin().lines().map(|line| {
        let line = line.unwrap();
        println!("{line}");

        let (pattern, groups) = line.split_once(' ').unwrap();
        let pattern = pattern.to_owned() + "?" + pattern + "?" + pattern + "?"+ pattern + "?"+ pattern;

        let groups: Vec<usize> = groups.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        let gl = groups.len();
        let groups: Vec<usize> = groups.into_iter().cycle().take(gl * 5).collect();

        let mut chunks: Vec<Chunk> = vec![];

        let mut i: usize = 0;
        while i < pattern.len() {
            match pattern.chars().nth(i).unwrap() {
                '#' => {
                    let len = pattern[i..].chars().take_while(|&ch| ch == '#').count();

                    chunks.push(Chunk { kind: C::Broken, start: i, len });
                    i += len;
                },
                '?' => {
                    let len = pattern[i..].chars().take_while(|&ch| ch == '?').count();

                    chunks.push(Chunk { kind: C::Maybe, start: i, len });
                    i += len;
                }
                '.' => {
                    let len = pattern[i..].chars().take_while(|&ch| ch == '.').count();
                    chunks.push(Chunk { kind: C::Fine, start: i, len });
                    i += len;
                }
                _ => {
                    i += 1;
                }
            }
        }

        // Add terminating Chunk::Fine to simplify next element checks
        if chunks.last().unwrap().kind != C::Fine {
            chunks.push(Chunk { kind: C::Fine, start: pattern.len(), len: 0 });
        }

        ways(0, &chunks, &groups, 0)
    }).sum();

    println!("{result}");
}
