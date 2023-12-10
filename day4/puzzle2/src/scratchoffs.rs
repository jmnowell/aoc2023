use std::{collections::HashSet, ops::Index, borrow::BorrowMut};

use regex::Regex;

#[derive(Debug)]
pub struct ScratchCard {
    pub id: usize,
    winning_numbers: HashSet<usize>,
    scratched_numbers: HashSet<usize>
}

impl ScratchCard {
    pub fn new(data: &str) -> Option<ScratchCard> {
        let game_re = Regex::new(r"Card\t| [0-9]+:").unwrap();
        let mut game_id: i64 = -1;

        match game_re.captures(data) {
            Some(val) => {
                game_id = val.iter().next()
                             .unwrap()
                             .as_ref()
                             .unwrap()
                             .as_str()
                             .trim()
                             .strip_suffix(':')
                             .unwrap_or_else(|| val.iter().next().unwrap().as_ref().unwrap().as_str().trim())
                             .parse::<i64>()
                             .unwrap_or(-1);
            },

            None => (),
        }

        if game_id == -1 {
            return None;
        }

        let throwaway: Vec<&str> = data.split(':').collect();
        let numbers: Vec<&str> = throwaway[1].split('|').collect();
        let winning_numbers: HashSet<usize> = numbers[0].split_ascii_whitespace()
                                                    .map(|digits| digits.parse::<usize>().unwrap())
                                                    .collect();

        let scratch_numbers: HashSet<usize> = numbers[1].split_ascii_whitespace()
                                                    .map(|digits| digits.parse::<usize>().unwrap())
                                                    .collect();

        Some(ScratchCard {
            id: game_id as usize,
            winning_numbers: winning_numbers,
            scratched_numbers: scratch_numbers
        })
    }

    pub fn point_value(&self) -> usize {
        let count = self.winning_numbers.intersection(&self.scratched_numbers).count();

        if count > 1 {
            return usize::pow(2, count as u32 - 1);
        } else if count == 1 {
            return 1;
        }

        count
    }

    pub fn next_cards(&self) -> Option<Vec<usize>> {
        let count = self.winning_numbers.intersection(&self.scratched_numbers).count();

        if count > 1 {
            let mut next_cards: Vec<usize> = Vec::new();

            for i in 0..count {
                next_cards.push(self.id + (i+1));
                
            }

            return Some(next_cards);
        } else if count == 1 {
            return Some(vec![self.id + 1]);
        }

        None
    }
}

#[derive(Debug)]
pub struct ScratchGames {
    games: Vec<ScratchCard>,
    copied_cards: Vec<usize>,
}

impl ScratchGames {
    pub fn new() -> ScratchGames {
        ScratchGames{
            games: Vec::new(),
            copied_cards: Vec::new()
        }
    }

    pub fn insert_games(&mut self, games: &mut Vec<ScratchCard>) {
        for _ in 0..games.len() {
            self.copied_cards.push(0);
        }

        self.games.append(games);
    }

    pub fn copied_count(&self) -> usize {
        self.copied_cards.iter()
                         .fold(0, |acc, num| acc + num)
    }

    pub fn original_count(&self) -> usize {
        self.games.len()
    }

    pub fn find_copies(&mut self) {
        for g in self.games.iter() {
            let mut next: Vec<usize> = Vec::new();

            // preload the next stack with all
            // the next cards from the game
            match g.next_cards() {
                Some(val) => {
                    for c in val {
                        next.push(c);
                    }
                },
                None => continue,
            }

            loop {
                match next.pop() {
                    Some(id) => {
                        // increment it's copied cards bucket
                        let target_id = id - 1;
                        self.copied_cards[target_id] += 1;

                        // fetch the next cards for this id
                        // and push it onto the stack
                        // if None - continue with this loop
                        match self.games.index(target_id).next_cards() {
                            Some(val) => {
                                for c in val {
                                    next.push(c);
                                }
                            }, 
                            None => { continue; }
                        }

                    },
                    None => { break; }
                }
            }
        }
    }


}