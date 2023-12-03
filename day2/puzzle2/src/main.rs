mod game_info;
use game_info::*;

use std::fs::read_to_string;

// The shamelessly stolen read_lines func does work.
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn find_games(games: &Vec<GameInfo>, max_red: u16, max_green: u16, max_blue: u16) -> usize {
    games.iter()
         .filter(|info| {
            let red_ok = info.red.iter().filter(|&&val| val <= max_red).count();
            let blue_ok = info.blue.iter().filter(|&&val| val <= max_blue).count();
            let green_ok = info.green.iter().filter(|&&val| val <= max_green).count();

            red_ok == info.red.len() &&
            blue_ok == info.blue.len() &&
            green_ok == info.green.len()
         })
         .fold(0, |acc, info| acc + info.id)
}

fn main() {
    let input = read_lines("input.txt");
    let mut games: Vec<GameInfo> = Vec::new();

    for line in input {
        let res = GameInfo::new(&line);

        match res {
            Some(val) => { 
                games.push(val);
            },
            None => (),
        }
    }

    println!("Result: {}", find_games(&games, 12, 13, 14));
}


#[cfg(test)]
#[test]
fn given_case() {
    let test_input = Vec::from([
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    ]);

    let mut test_result: Vec<GameInfo> = Vec::new();

    for line in test_input {
        let res = GameInfo::new(line);

        match res {
            Some(val) => test_result.push(val),
            None => (),
        }
    }

    assert_eq!(test_result.len(), 5);
    assert_eq!(find_games(&test_result, 12, 13, 14), 8);
}