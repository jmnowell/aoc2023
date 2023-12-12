use binary_search::{ binary_search, Direction };

use std::ops::Range;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub struct MapData {
    pub source: usize,
    source_range: Vec<usize>,
    dest_range: Vec<usize>,
}

impl MapData {
    pub fn new(source: usize, dest: usize, range: usize) -> MapData {
        MapData{
            source: source,
            source_range: Range{ start: source, end: source + range }.collect(),
            dest_range: Range{ start: dest, end: dest + range }.collect(),
        }
    }

    pub fn get_destination(&self, source: usize) -> Option<usize> {
        if !self.source_range.contains(&source) {
            return None;
        }

        // because the ranges are same sized, we can extract
        // the index from the source as the index into the dest
        // and pull the value
        let mut source_idx = 0;

        for (val, i) in self.source_range.iter().enumerate() {
            if val == source {
                source_idx = *i;
                break;
            }
        }

        Some(self.dest_range[source_idx])
    }
}

#[derive(Debug)]
pub struct MapTable {
    map: HashMap<usize, MapData>,
}

impl MapTable {
    pub fn new() -> MapTable {
        MapTable { map: HashMap::new(), }
    }

    pub fn insert(&mut self, data: MapData) -> bool {
        if !self.map.contains_key(&data.source) {
            self.map.insert(data.source, data);
            return true;
        }

        false
    }

    pub fn find_mapping(&self, source: usize) -> Option<usize> {
        // three cases
        //      1) we hit it dead on
        //      2) it's lower than the middle
        //      3) it's higher than the middle
        //
        //  1) is the fastest outta here
        if self.map.contains_key(&source) {
            return self.map.get(&source).unwrap().get_destination(source);
        }

        // setup binary search
        let keys: Vec<_> = self.map.keys().cloned().collect();
        let pivot = keys[keys.len() / 2];

        let (low, high) = binary_search((0, 90), 
                                    (keys.len(), ()), 
                                    |i| {
                                        if keys[i] < pivot {
                                            Direction::Low(pivot)
                                        } else {
                                            Direction::High(())
                                        }
                                    });

        // check the low first
        let low_dest = self.map.get(&low.0).unwrap().get_destination(source);
        
        match low_dest {
            Some(_) => {
                return low_dest;
            },
            None => (),
        }

        // return hi_dest - has to be it, right?
        self.map.get(&high.0).unwrap().get_destination(source)
    }
}