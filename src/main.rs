extern crate slab;
use slab::{Slab, VacantEntry};

fn main() {
    const CAPACITY: usize = 1024;
    let mut slab = Slab::with_capacity(CAPACITY);

    let hello_key = slab.insert("hello");
    let world_key = slab.insert("world");
    println!("hello_key -> '{}'", slab[hello_key]);
    println!("world_key -> '{}'", slab[world_key]);

    let data_key = {
        let entry = slab.vacant_entry();
        fill_some_data(entry)
    };
    println!("data_key -> '{}'", slab[data_key]);

    for (key, val) in &slab {
        println!("{} -> {}", key, val);
    }

    if slab.len() != slab.capacity() {
        slab.insert("the slab is not at capacity yet");
    }
}

fn fill_some_data(entry: VacantEntry<&str>) -> usize {
    let data = "Some data";
    let key = entry.key();
    entry.insert(data);
    key
}