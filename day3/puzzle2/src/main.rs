mod schematic_helpers;
use schematic_helpers::*;

use std::fs::File;
use std::io::{prelude::*, BufReader};

fn parse_file(path: &str) -> Map {
    let mut map = Map::new();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for (x_coord, line) in reader.lines().enumerate() {
        let as_str = line.unwrap(); // ok to panic
        map.height += 1; // increment the height
        let mut width = 0;
        let mut part_no = String::new();
        let mut last_y_coord: usize = 0;

        for (y_coord, c) in as_str.chars().enumerate() {
            width += 1;
            last_y_coord = y_coord;

            if c.is_digit(10) {
                part_no.push(c);
            } else {
                // we don't have an ascii digit,
                // so we need to parse the number
                // and then save it's x_coord and y_coord
                // as well as the length.
                {
                    let map_borrow = &mut map;

                    if part_no.len() > 0 {
                        let key = x_coord as i64;

                        map_borrow.parts.entry(key)
                                        .and_modify(|v| {
                                            v.push(Part { 
                                                number: part_no.parse::<usize>().unwrap(), 
                                                len: part_no.len(), 
                                                x: x_coord as i64, 
                                                y: y_coord as i64 - 1, 
                                            });
                                        }).or_insert(vec![Part{
                                            number: part_no.parse::<usize>().unwrap(), 
                                                len: part_no.len(), 
                                                x: x_coord as i64, 
                                                y: y_coord as i64 - 1,
                                        }]);
                        part_no.clear();
                    }
                }

                {
                    let map_borrow = &mut map;

                    if c == '*' {
                        // save the symbol
                        let key = x_coord as i64;
                        map_borrow.gears.push(Symbol{
                            symbol: c,
                            x: x_coord as i64,
                            y: y_coord as i64,
                        });
                    }
                }
            }
        }

        // one final fuck you from AoC2023
        {
            let map_borrow = &mut map;

            if part_no.len() > 0 {
                let key = x_coord as i64;

                map_borrow.parts.entry(key)
                                .and_modify(|v| {
                                    v.push(Part { 
                                        number: part_no.parse::<usize>().unwrap(), 
                                        len: part_no.len(), 
                                        x: x_coord as i64, 
                                        y: last_y_coord as i64 - 1, 
                                    });
                                }).or_insert(vec![Part{
                                    number: part_no.parse::<usize>().unwrap(), 
                                        len: part_no.len(), 
                                        x: x_coord as i64, 
                                        y: last_y_coord as i64 - 1,
                                }]);
                part_no.clear();
            }
        }

        if map.width <= width {
            map.width = width;
        }
    }

    // ?? seems to be an issue?
    map.width -= 1;
    map.height -= 1;
    map
}

fn main() {
    let map = parse_file("test_input.txt");
    let sum = map.compute_sum();

    println!("Result: {}", sum);
}

#[cfg(test)]
#[test]
fn given_case() {
    let map = parse_file("test_input.txt");
    let sum = map.compute_sum();
    assert_eq!(sum, 467835);
}
