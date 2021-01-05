use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::VecDeque;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let (mut deck1, mut deck2) = parse_input(&lines);

    // simulate a game of Combat
    while deck1.len() > 0 && deck2.len() > 0 {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else if card1 < card2 {
            deck2.push_back(card2);
            deck2.push_back(card1);
        } else {
            unreachable!();
        }
    }

    let winning_deck = if deck1.len() > 0 {
        deck1
    } else if deck2.len() > 0 {
        deck2
    } else {
        unreachable!()
    };

    // calculate score
    let score: i64 = winning_deck.iter().rev().enumerate().map(|(i, card)| card * (i as i64 + 1)).sum();
    println!("Score: {}", score);

    Ok(())
}

type Deck = VecDeque<i64>;

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
