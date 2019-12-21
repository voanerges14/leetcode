struct Solution{}

use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut letters = HashMap::new();

        s.chars().for_each(|_ch| {
            *letters.entry(_ch).or_insert(0) += 1;
        });

        let mut res = letters.into_iter().collect::<Vec<_>>();
        res.sort_by_key(|r| 0 - r.1 as i32);
        res.iter().map(|r| r.0.to_string().repeat(r.1)).collect()
    }
}

fn main() {
    println!("{:?}", Solution::frequency_sort("some string to be sorted".to_owned()));
}
