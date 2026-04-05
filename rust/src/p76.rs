use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut char_counts = HashMap::new();
        for c in t.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }
        let mut remaining_chars = t.len();
        let s_chars = s.chars().collect::<Vec<char>>();

        let mut low = 0;
        let mut high = 0;
        let mut ans: Option<(usize, usize)> = None;
        println!("{:?} {:?}", s, t);
        while high < s_chars.len() {
            if let Some(count) = char_counts.get_mut(&s_chars[high]) {
                *count -= 1;
                println!("{:?} count", count);
                if *count >= 0 {
                    remaining_chars -= 1;
                }
            }

            println!(
                "{:?} low {:?} high {:?} rem {:?}",
                char_counts, low, high, remaining_chars
            );

            high += 1;

            while remaining_chars == 0 {
                if let Some((ans_low, ans_high)) = ans {
                    let ans_diff = ans_high - ans_low;
                    let new_diff = high - low;
                    if new_diff < ans_diff {
                        ans = Some((low, high));
                    }
                } else {
                    ans = Some((low, high));
                }

                if let Some(count) = char_counts.get_mut(&s_chars[low]) {
                    *count += 1;
                    if *count > 0 {
                        remaining_chars += 1;
                    }
                }

                low += 1;
            }

            while low < s_chars.len() && char_counts.get(&s_chars[low]).is_none() {
                low += 1;
            }
            high = high.max(low);
        }

        let ans = ans.unwrap_or_else(|| (0, 0));
        s[ans.0..ans.1].to_owned()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned()),
            "BANC".to_owned()
        );
    }
}
