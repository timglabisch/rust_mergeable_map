mod mapmerge;

use std::collections::HashMap;
use mapmerge::MergeMap;

fn main() {

    

    let mut map_merge = MergeMap::new();

    for rounds in 0..10 {
        let mut some_map = HashMap::new();
        for i in 0..1_000_000 {
            some_map.insert(i, i*2);
        }

        map_merge.merge(some_map, rounds);
    }


    println!("Hello, world!");
}
