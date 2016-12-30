use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::hash::Hash;

pub struct MergeMap {
    size: usize,
    data: HashMap<u32, Vec<u32>>
}

impl MergeMap {

    pub fn new() -> MergeMap {
        MergeMap {
            size: 31,
            data: HashMap::new()
        }
    }

    pub fn merge(&mut self, mut map : HashMap<u32, u32>, day : usize) {
        
        let s = self.size;

        for (k, v) in map.drain() {
            let entry = self.data.entry(k).or_insert_with(|| vec![0; s]);
            entry[day] = v;
        }

    }

}