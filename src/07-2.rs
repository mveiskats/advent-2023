use std::io;
use std::collections::HashMap;

fn card_rank(ch: char) -> u8 {
    match ch {
        'A'=>1,
        'K'=>2,
        'Q'=>3,
        'T'=>4,
        '9'=>5,
        '8'=>6,
        '7'=>7,
        '6'=>8,
        '5'=>9,
        '4'=>10,
        '3'=>11,
        '2'=>12,
        'J'=>13,
        _=>panic!("Unknown card")
    }
}

fn card_frequencies(hand: &Vec<char>) -> HashMap<char, usize> {
    let mut frequencies: HashMap<char, usize> = HashMap::new();

    for &ch in hand {
        *frequencies.entry(ch).or_insert(0) += 1;
    }

    frequencies
}

fn hand_rank(hand: &String) -> u8 {
    let jokers = hand.chars().filter(|&ch| ch == 'J').count();
    let cards: Vec<char> = hand.chars().filter(|&ch| ch != 'J').collect();

    let freq = card_frequencies(&cards);
    let fives = freq.values().filter(|&&f| f + jokers == 5).count();
    let fours = freq.values().filter(|&&f| f + jokers == 4).count();
    let threes = freq.values().filter(|&&f| f + jokers == 3).count();
    let twos = freq.values().filter(|&&f| f + jokers == 2).count();
    let twos_without_jokers = freq.values().filter(|&&f| f == 2).count();

    // If you stare into the abyss, the abyss stares back at you
    if fives > 0 || jokers == 5 { 1 }
    else if fours > 0 { 2 }
    else if (threes == 1 && twos == 1 && jokers == 0) ||
        (twos_without_jokers == 1 && jokers == 2) ||
        (twos_without_jokers == 2 && jokers == 1) { 3 }
    else if threes > 0 { 4 }
    else if twos == 2 && jokers == 0 { 5 }
    else if twos > 0 { 6 }
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
