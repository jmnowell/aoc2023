use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn extract_number(line: &str) -> u32 {
    let mut first = 'z';
    let mut second = 'z';

    for c in line.chars() {
        if c.is_digit(10) {
            if first == 'z' {
                first = c;
            } else {
                second = c;
            }
        }
    }

    let mut numbers = String::new();

    numbers.push(first);

    if second == 'z' {
        numbers.push(first);
    } else {
        numbers.push(second);
    }
    
    numbers.parse::<u32>().unwrap()
}

fn main() {
    let mut sum: u32 = 0;
    let test_input = read_lines("input.txt");

    for line in test_input {
        sum = sum + extract_number(&line);
    }

    println!("Result: {}", sum);
}

#[cfg(test)]
#[test]
fn given_case() {
    let test_input = Vec::from([
        "1abc2",
        "pqr3stu8vwx",
        "a1b2c3d4e5f",
        "treb7uchet"
    ]);

    let mut sum: u32 = 0;

    for line in test_input {
        sum = sum + extract_number(&line);
    }

    assert_eq!(sum, 142);
}
