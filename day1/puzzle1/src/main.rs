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
        .filter(|c| c.is_digit(10))
        .collect::<Vec<_>>();

    // only take the first and the last
    // NOTE: this will panic if the line doesn't contain any digits
    let _numbers = format!("{}{}", digits[0], digits[digits.len() - 1]);

    // here's an approach that won't panic
    let mut digits_iter = digits.into_iter();
    let first = digits_iter.next().unwrap_or('0');
    let last = digits_iter.last().unwrap_or(first);

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
