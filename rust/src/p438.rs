use std::collections::HashMap;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut base_map = HashMap::<char, usize>::new();
        for ele in p.chars() {
            *base_map.entry(ele).or_insert(0) += 1;
        }

        let s_chars: Vec<char> = s.chars().collect();
        let mut low = 0;
        let mut high = 0;
        let mut total_count = p.len();
        let mut curr_map = base_map.clone();
        let mut ans = vec![];
        while high < s.len() {
            let Some(rem_count) = curr_map.get_mut(&s_chars[high]) else {
                low = high + 1;
                high = low;
                total_count = p.len();
                curr_map = base_map.clone();
                continue;
            };

            if *rem_count == 0 {
                *curr_map.get_mut(&s_chars[low]).unwrap() += 1;
                low += 1;
                total_count += 1;
                continue;
            }

            total_count -= 1;
            *rem_count -= 1;

            if total_count == 0 {
                ans.push(low as i32);
                *curr_map.get_mut(&s_chars[low]).unwrap() += 1;
                total_count += 1;
                low += 1;
            }

            high += 1;
        }

        ans
    }
}

struct Solution;
