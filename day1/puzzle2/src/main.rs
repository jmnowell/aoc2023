use std::fs::read_to_string;
use std::collections::HashMap;

// shamelessly stolen from the rust book
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn check_number(input: &str) -> Option<char> {
    // create hashtable for quick lookups
    // from one to nine to the digits
    let digits_table = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    for key in digits_table.keys() {
        if input.contains(*key) {
            return Some(digits_table[key]);
        }
    }

    None
}

fn extract_number(line: &str) -> u32 {
    let sentinel = 'z';
    let mut first = sentinel;
    let mut second = sentinel;
    let mut potential_digit = String::new();

    for c in line.chars() {
        if c.is_digit(10) {
            potential_digit.clear();

            if first == sentinel {
                first = c;
            } else {
                second = c;
            }
        } else {
            // add the char to the potential_digit
            // and then test it against the hash table
            potential_digit.push(c);

            match check_number(&potential_digit) {
                Some(val) =>
                    {
                        // on attempt 1, I did not 
                        // see the trick - boo
                        potential_digit.clear();
                        potential_digit.push(c);

                        if first == sentinel {
                            first = val;
                        } else {
                            second = val;
                        }
                    },
                None => (),
            }
        }
    }

    let mut numbers = String::new();
    numbers.push(first);

    if second == sentinel {
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
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen"
    ]);

    let mut sum: u32 = 0;

    for line in test_input {
        sum = sum + extract_number(line);
    }

    assert_eq!(sum, 281);
}
