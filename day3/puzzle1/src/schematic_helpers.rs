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
    pub numbers: Vec<Part>,
    pub symbols: HashMap<i64, Vec<Symbol>>,
    pub width: i64,
    pub height: i64,
}

impl Map {
    pub fn new() -> Map {
        Map {
            numbers: Vec::new(),
            symbols: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn compute_sum(self: &Map) -> usize {
        // let mut collected_nums: HashSet<usize> = HashSet::new();
        let mut collected_nums: Vec<usize> = Vec::new();

        // for each element in the numbers, we first look up it's 
        // x coordinate and then generate it's adjacent row
        // edge cases are for the ends of the input, meaning 1 and n-1
        // cannot be decremented or incremented respectively
        for part in self.numbers.iter() {
            let coords = AdjCoords::from_part(&part, self.width, self.height);

            // search for the following in this order:
            //  1) current row (x_coord) and y_beh, y_forward
            //  2) backward row (if != -1)
            //  3) forward row (if != -1)
            if self.symbols.contains_key(&part.x) {
                for symbol in self.symbols.get(&part.x).unwrap().iter() {
                    // if the symbol is in the current row, it is only in
                    // one of two possible places
                    //  1) next to the y_coord (y_coord forward)
                    //  2) a in front of the number, where:
                    //      symbol.y = part.y - (part.len + 1)
                    if symbol.y == coords.y_behind || symbol.y == coords.y_forward {
                        collected_nums.push(part.number);
                    }
                }
            }

            if self.symbols.contains_key(&coords.x_behind) {
                // for this case and 3), it is a little trickier
                // the symbol could be:
                //  1) y_forward as above.
                //  2) between y_behind and y_forward
                for symbol in self.symbols.get(&coords.x_behind).unwrap().iter() {
                    if symbol.y >= coords.y_behind && symbol.y <= coords.y_forward {
                        collected_nums.push(part.number);
                    }
                }   
            }

            if self.symbols.contains_key(&coords.x_forward) {
                for symbol in self.symbols.get(&coords.x_forward).unwrap().iter() {
                    if symbol.y >= coords.y_behind && symbol.y <= coords.y_forward {
                        collected_nums.push(part.number);
                    }
                }   
            }
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