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
            Some(val) => cards.push(val),
            None => println!("Did not find a card value"),
        }
    }

    let mut sum: usize = 0;

    for c in cards {
        sum = sum + c.point_value();
    }

    println!("Result: {}", sum);
}

#[cfg(test)]
#[test]
fn given_case() {
    let lines = read_file("test_input.txt");
    let mut cards: Vec<ScratchCard> = Vec::new();

    for ln in lines {
        match ScratchCard::new(&ln) {
            Some(val) => cards.push(val),
            None => println!("Did not find a card value."),
        }
    }

    let mut sum: usize = 0;

    for c in cards {
        sum = sum + c.point_value();
    }

    assert_eq!(sum, 13);
}
