use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut hashmap = HashMap::new();
        matches.iter().for_each(|data| {
            hashmap.entry(data[0]).or_insert(0);
            hashmap.entry(data[1]).and_modify(|c| *c += 1).or_insert(1);
        });
        let mut zero = vec![];
        let mut one = vec![];
        hashmap.iter().for_each(|(&k, &v)| {
            if v == 0 {
                zero.push(k);
            } else if v == 1 {
                one.push(k);
            };
        });
        zero.sort();
        one.sort();
        vec![zero, one]
    }
}
