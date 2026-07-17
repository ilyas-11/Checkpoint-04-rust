use std::collections::HashMap;

pub fn smallest(h: HashMap<&str, i32>) -> i32 {
    let mut min = i32::MAX;

    for value in h.values() {
        if *value < min {
            min = *value;
        }
    }

    min
}