fn extract_number(line: &String) -> u32 {
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
    // do a little test!
    let mut input: Vec<String> = Vec::new();

    input.push(String::from("1abc2"));
    input.push(String::from("pqr3stu8vwx"));
    input.push(String::from("a1b2c3d4e5f"));
    input.push(String::from("treb7uchet"));

    let mut sum: u32 = 0;

    for line in input {
        let val = extract_number(&line);
        println!("DEBUG: extract_number {}", val);
        sum = sum + extract_number(&line);
    }

    println!("Result: {}", sum);
}
