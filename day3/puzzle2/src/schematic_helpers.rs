use std::collections::{HashMap};

#[derive(Debug)]
pub struct Part {
    pub number: usize,
    pub len: usize,
    pub x: i64,
    pub y: i64, 
}

#[derive(Debug)]
pub struct Symbol {
    pub symbol: char,
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
pub struct Map {
    pub gears: Vec<Symbol>,
    pub parts: HashMap<i64, Vec<Part>>,
    pub width: i64,
    pub height: i64,
}

impl Map {
    pub fn new() -> Map {
        Map {
            gears: Vec::new(),
            parts: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn compute_sum(self: &Map) -> usize {
        let mut collected_nums: Vec<usize> = Vec::new();

        for gear in self.gears.iter() {
            let coords = AdjCoords::from_gear(&gear, self.width, self.height);
            let mut ratios: Vec<usize> = Vec::new();

            println!("DEBUG: symbol (x, y): {}, {} (x_b, x_f, y_b, y_f): {}, {}, {}, {}", gear.x, gear.y, coords.x_behind, coords.x_forward, coords.y_behind, coords.y_forward);

            // search for the following in this order:
            //  1) current row (x_coord) and y_beh, y_forward
            //  2) backward row (if != -1)
            //  3) forward row (if != -1)
            if self.parts.contains_key(&gear.x) {
                for part in self.parts.get(&gear.x).unwrap().iter() {
                    if part.y == coords.y_behind {
                        ratios.push(part.number);
                    }

                    if part.x == coords.x_behind {
                        ratios.push(part.number);
                    }
                }
            }

            // subsequent searches are harder, as the gear symbol could
            // be adjacent to AdjCoords::from_part
            if self.parts.contains_key(&coords.x_behind) {
                for part in self.parts.get(&coords.x_behind).unwrap().iter() {
                    let part_coords = AdjCoords::from_part(part, self.width, self.height);

                    println!("DEBUG: part no: {}, (x, y): {}, {}, (x_b, x_f, y_b, y_f): {}, {}, {}, {}", part.number, part.x, part.y, part_coords.x_behind, part_coords.x_forward, part_coords.y_behind, part_coords.y_forward);
                    
                    if coords.y_behind >= part_coords.y_behind  && gear.y <= part_coords.y_forward {
                        ratios.push(part.number);
                    }
                }   
            }

            if self.parts.contains_key(&coords.x_forward) {
                for part in self.parts.get(&coords.x_forward).unwrap().iter() {
                    let part_coords = AdjCoords::from_part(part, self.width, self.height);

                    println!("DEBUG: part no: {}, (x, y): {}, {}, (x_b, x_f, y_b, y_f): {}, {}, {}, {}", part.number, part.x, part.y, part_coords.x_behind, part_coords.x_forward, part_coords.y_behind, part_coords.y_forward);

                    if coords.y_behind >= part_coords.y_behind && gear.y <= part_coords.y_forward {
                        ratios.push(part.number);
                    }
                }   
            }

            
            if ratios.len() > 2 {
                collected_nums.push(ratios.iter().fold(1, |acc, num| { acc * num}));
            }
            
            ratios.clear();
        }

        collected_nums.iter().fold(0, |acc, num| { acc + num })
    }
}

#[derive(Debug)]
pub struct AdjCoords {
    pub x_behind: i64,
    pub y_behind: i64,
    pub x_forward: i64,
    pub y_forward: i64,
}

impl AdjCoords {
    pub fn from_gear(symbol: &Symbol, max_width: i64, max_height: i64) -> AdjCoords {
        let mut x_beh: i64;
        let mut x_for: i64;

        x_beh = symbol.x - 1;
        x_for = symbol.x + 1;

        if x_beh < 0 {
            x_beh = -1;
        }

        if x_for > max_height {
            x_for = -1;
        }

        let mut y_beh: i64;
        let mut y_for: i64;

        y_beh = symbol.y - 1;
        y_for = symbol.y + 1;

        if y_beh < 0 {
            y_beh = 0;
        }


        if y_for > max_width {
            y_for = -1;
        }

        AdjCoords { 
            x_behind: x_beh, 
            y_behind: y_beh, 
            x_forward: x_for, 
            y_forward: y_for 
        }
    }

    pub fn from_part(part: &Part, max_width: i64, max_height: i64) -> AdjCoords {
        let mut x_beh: i64;
        let mut x_for: i64;

        x_beh = part.x - 1;
        x_for = part.x + 1;

        if x_beh < 0 {
            x_beh = -1;
        }

        if x_for > max_height {
            x_for = -1;
        }

        let mut y_beh: i64;
        let mut y_for: i64;

        y_beh = part.y - part.len as i64;

        if y_beh < 0 {
            y_beh = 0;
        }

        y_for = part.y + 1;

        if y_for > max_width {
            y_for = y_beh + part.len as i64;
        }

        AdjCoords { 
            x_behind: x_beh, 
            y_behind: y_beh, 
            x_forward: x_for, 
            y_forward: y_for 
        }
    }
}