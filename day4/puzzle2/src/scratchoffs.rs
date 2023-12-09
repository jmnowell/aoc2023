use std::collections::HashSet;
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

    pub fn point_value(self) -> usize {
        let count = self.winning_numbers.intersection(&self.scratched_numbers).count();

        if count > 1 {
            return usize::pow(2, count as u32 - 1);
        } else if count == 1 {
            return 1;
        }

        count
    }
}