mod scratchoffs;
use scratchoffs::*;

use std::fs::read_to_string;

fn read_file(path: &str) -> Vec<String>{
    read_to_string(path)
                .unwrap()
                .lines()
                .map(String::from)
                .collect()
}

fn main() {
    let lines = read_file("input.txt");
    let mut cards: Vec<ScratchCard> = Vec::new();

    for ln in lines {
        match ScratchCard::new(&ln) {
            Some(val) => {
                cards.push(val);
            },
            None => println!("Did not find a card value"),
        }
    }

    // create the Games
    let mut games = ScratchGames::new();
    games.insert_games(&mut cards);
    games.find_copies();
    let sum = games.copied_count() + games.original_count();

    println!("Result: {}", sum);
}

#[cfg(test)]
#[test]
fn given_case() {
    let lines = read_file("test_input.txt");
    let mut cards: Vec<ScratchCard> = Vec::new();

    for ln in lines {
        match ScratchCard::new(&ln) {
            Some(val) => {
                cards.push(val);
            },
            None => println!("Did not find a card value."),
        }
    }

    let mut games = ScratchGames::new();
    games.insert_games(&mut cards);
    games.find_copies();
    let sum = games.copied_count() + games.original_count();

    assert_eq!(sum, 30);
}

#[test]
fn actual_result() {
    let lines = read_file("input.txt");
    let mut cards: Vec<ScratchCard> = Vec::new();

    for ln in lines {
        match ScratchCard::new(&ln) {
            Some(val) => {
                cards.push(val);
            },
            None => println!("Did not find a card value"),
        }
    }

    // create the Games
    let mut games = ScratchGames::new();
    games.insert_games(&mut cards);
    games.find_copies();
    let sum = games.copied_count() + games.original_count();

    assert_eq!(sum, 13261850);
}
