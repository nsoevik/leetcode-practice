impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // DP
        // If s[i..j] is a palindrome, then s[i-1..=j] is if the outer are equal
        // a b b a
            // 1 0 0 0
            // 0 1 2 0
            // 0 0 1 0
            // 0 0 0 1
        let s_len = s.len();
        let mut dp = vec![vec![0; s_len]; s_len];
        let chars = s.chars().collect::<Vec<char>>();

        let mut ans = &s[0..1];
        for i in 0..s_len {
            dp[i][i] = 1;

            if i + 1 < s_len && chars[i] == chars[i + 1] {
                dp[i][i + 1] = 2;
                ans = &s[i..=i+1]
            }
        }

        let mut max_so_far = 1;
        for dist in 2..s_len {
            for start in 0..s_len - 1 {
                let end = start + dist;
                if end < s_len && chars[start] == chars[end] && dp[start+1][end-1] > 0 {
                    dp[start][end] = dp[start+1][end-1] + 2;
                    if dp[start][end] > max_so_far {
                        println!("reached max {:?} {:?}", start, end);
                        max_so_far = dp[start][end];
                        ans = &s[start..=end];
                    }
                }
            }
        }

        println!("{:?}", dp);

        ans.to_owned()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
