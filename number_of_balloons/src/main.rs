struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut letters_count = HashMap::new();
        let mut letters = ['b', 'a', 'l', 'o', 'n'];

        text.chars().for_each(|_ch| {
            if letters.contains(&_ch) {
                *letters_count.entry(_ch).or_insert(0) += 1;
            }
        });

        let l = *letters_count.get(&'l').unwrap_or(&0);
        let o = *letters_count.get(&'o').unwrap_or(&0);

        let min_of_max = if l < o { l / 2 } else { o / 2 };

        letters_count.remove(&'o');
        letters_count.remove(&'l');

        let mut res = letters_count.into_iter().collect::<Vec<_>>();
        res.sort_by_key(|r| r.1 as i32);
        let min = res.get(0).unwrap_or(&(' ', 0)).1;
        if min < min_of_max { min } else { min_of_max }
    }
}

fn main() {
    println!("{:?}", Solution::max_number_of_balloons("loonbalxballpoon".to_owned()));
}
