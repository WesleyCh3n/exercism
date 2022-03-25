// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
// #![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut dict_mag: HashMap<&&str, i32> = HashMap::new();

    for word in magazine.into_iter() {
        *dict_mag.entry(word).or_insert(0) += 1;
    }

    for word in note {
        // *dict_mag.entry(word).or_insert(-1) += 1;
        if let true = dict_mag.contains_key(word) {
            if dict_mag.insert(word, dict_mag[word] - 1).unwrap() - 1 < 0 {
                return false;
            };
        } else {
            return false;
        }
    }

    true
}
