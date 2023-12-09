use std::io;
use std::collections::HashMap;

fn card_rank(ch: char) -> u8 {
    match ch {
        'A'=>1,
        'K'=>2,
        'Q'=>3,
        'J'=>4,
        'T'=>5,
        '9'=>6,
        '8'=>7,
        '7'=>8,
        '6'=>9,
        '5'=>10,
        '4'=>11,
        '3'=>12,
        '2'=>13,
        _=>panic!("Unknown card")
    }
}

fn card_frequencies(hand: &String) -> HashMap<char, u8> {
    let mut frequencies: HashMap<char, u8> = HashMap::new();

    for ch in hand.chars() {
        *frequencies.entry(ch).or_insert(0) += 1;
    }

    frequencies
}

fn hand_rank(hand: &String) -> u8 {
    let freq = card_frequencies(hand);
    let fives = freq.values().filter(|&&f| f == 5).count();
    let fours = freq.values().filter(|&&f| f == 4).count();
    let threes = freq.values().filter(|&&f| f == 3).count();
    let twos = freq.values().filter(|&&f| f == 2).count();

    if fives == 1 { 1 }
    else if fours == 1 { 2 }
    else if threes == 1 && twos == 1 { 3 }
    else if threes == 1 { 4 }
    else if twos == 2 { 5 }
    else if twos == 1 { 6 }
    else { 7 }
}

fn main() {
    let mut hands: Vec<(String, usize)> = io::stdin().lines().map(|line| {
        let line = line.unwrap();
        let parts: Vec<_> = line.split(' ').collect();
        let cards = String::from(parts[0]);
        let bet = parts[1].parse::<usize>().unwrap();

        (cards, bet)
    }).collect();

    hands.sort_by(|a, b| {
        let a_hand = &a.0;
        let a_rank = hand_rank(a_hand);
        let a_card_ranks = a_hand.chars().map(|ch| card_rank(ch));

        let b_hand = &b.0;
        let b_rank = hand_rank(b_hand);
        let b_card_ranks = b_hand.chars().map(|ch| card_rank(ch));

        a_rank.cmp(&b_rank).then(a_card_ranks.cmp(b_card_ranks))
    });

    let winnings: usize = hands.into_iter().map(|(_cards, bet)| bet)
        .rev()
        .enumerate()
        .map(|(i, bet)| (i + 1) * bet)
        .sum();

    println!("Winnings: {winnings}");
}
