use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn extract_number(line: &str) -> u32 {
    // let mut first = 'z';
    // let mut second = 'z';

    // for c in line.chars() {
    //     if c.is_digit(10) {
    //         if first == 'z' {
    //             first = c;
    //         } else {
    //             second = c;
    //         }
    //     }
    // }

    // let mut numbers = String::new();

    // numbers.push(first);

    // if second == 'z' {
    //     numbers.push(first);
    // } else {
    //     numbers.push(second);
    // }

    // collect all digits into a Vec<_>
    let digits = line
        .chars()
        .into_iter()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    // only take the first and the last
    let first = digits.first().unwrap_or(&0);
    let last = digits.last().unwrap_or(first);

    let numbers = format!("{first}{last}");

    numbers.parse::<u32>().unwrap()
}

fn main() {
    let mut sum: u32 = 0;
    let test_input = read_lines("input.txt");

    for line in test_input {
        sum = sum + extract_number(&line);
    }

    println!("Result: {}", sum);

    // another approach using iterators
    let sum: u32 = read_lines("input.txt")
        .iter()
        .map(String::as_str)
        .map(extract_number)
        .sum();

    println!("Result: {}", sum);
}

#[cfg(test)]
#[test]
fn given_case() {
    let test_input = Vec::from(["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]);

    let mut sum: u32 = 0;

    for line in test_input {
        sum = sum + extract_number(&line);
    }

    assert_eq!(sum, 142);
}
