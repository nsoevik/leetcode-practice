use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // "sp" ["s", "p"]
        // "" "s" "sp"
        // f   t   t
        let mut dp: Vec<bool> = vec![false; s.len()];
        let h: HashSet<String> = HashSet::from_iter(word_dict);

        for end in 1..=s.len() {
            if h.contains(&s[0..end]) {
                dp[end - 1] = true;
                continue;
            }

            for start in 1..end {
                if dp[start - 1] && h.contains(&s[start..end]) {
                    dp[end - 1] = true;
                    break;
                }
            }
        }

        dp[dp.len() - 1]
    }
}

struct Solution;
