use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let (mut deck1, mut deck2) = parse_input(&lines);

    let winner = recursive_combat_game( &mut deck1, &mut deck2);

    let winning_deck = match winner {
        Player::_1 => deck1,
        Player::_2 => deck2,
    };

    // calculate score
    let score: i64 = winning_deck.iter().rev().enumerate().map(|(i, card)| card * (i as i64 + 1)).sum();
    println!("Score: {}", score);

    Ok(())
}

#[derive(Debug,Clone,Copy)]
enum Player {
    _1,
    _2,
}

fn recursive_combat_game( deck1: &mut Deck, deck2: &mut Deck) -> Player {
    let mut previous_configs: HashSet<(Deck, Deck)> = HashSet::new();
    // simulate a game of Combat
    while deck1.len() > 0 && deck2.len() > 0 {
        let round_config = (deck1.clone(), deck2.clone());
        if previous_configs.contains(&round_config) {
            return Player::_1;
        }
        previous_configs.insert(round_config);
        recursive_combat_round( deck1, deck2);
    }

    if deck1.len() > 0 {
        Player::_1
    } else if deck2.len() > 0 {
        Player::_2
    } else {
        unreachable!()
    }
}

fn recursive_combat_round( deck1: &mut Deck, deck2: &mut Deck) {
    let card1 = deck1.pop_front().unwrap();
    let card2 = deck2.pop_front().unwrap();

    let winner = if deck1.len() as i64 >= card1 && deck2.len() as i64 >= card2 {
        // recursive games do not affect the current deck, except for deciding who won the round
        let mut deck1_slice_copy = deck1.iter().take(card1 as usize).cloned().collect();
        let mut deck2_slice_copy = deck2.iter().take(card2 as usize).cloned().collect();
        recursive_combat_game( &mut deck1_slice_copy, &mut deck2_slice_copy)
    } else {
        if card1 > card2 {
            Player::_1
        } else if card1 < card2 {
            Player::_2
        } else {
            unreachable!();
        }
    };
    match winner {
        Player::_1 => {
            deck1.push_back(card1);
            deck1.push_back(card2);
        }
        Player::_2 => {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
}

type Deck = VecDeque<i64>;
#[allow(dead_code)]
fn deck_str(deck: &Deck) -> String {
    let deck_str_vec: Vec<String> = deck.iter().map(|card| card.to_string()).collect();
    format!("[{}]", deck_str_vec.join(", ") )
}

fn parse_input(lines: &[String]) -> (Deck, Deck) {
    enum ParseState {
        Player1Label,
        Player1Card,
        Player2Label,
        Player2Card,
    }

    let mut state = ParseState::Player1Label;
    let mut player1_cards: Deck = Deck::new();
    let mut player2_cards: Deck = Deck::new();
    for line in lines {
        match state {
            ParseState::Player1Label => state = ParseState::Player1Card,
            ParseState::Player1Card => if line == "" {
                state = ParseState::Player2Label;
            } else {
                player1_cards.push_back(line.parse().unwrap());
            }
            ParseState::Player2Label => state = ParseState::Player2Card,
            ParseState::Player2Card => if line == "" {
                unreachable!();
            } else {
                player2_cards.push_back(line.parse().unwrap());
            }
        }
    }
    (player1_cards, player2_cards)
}
