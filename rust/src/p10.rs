impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // DP
        //   a a c
        // a t f f
        // * t t f
        // c f f t
        //
        //   - a c
        // - t f f
        // b f f f
        // * t t f
        // a f t f
        //
        //   - a c c b
        // - t f f f f
        // a f t f f f
        // c f f t f f
        // * f t t t f
        // b f f f f t

        let mut dp = vec![vec![false; s.len() + 1]; p.len() + 1];

        let p_chars: Vec<char> = p.chars().collect();
        let s_chars: Vec<char> = s.chars().collect();

        dp[0][0] = true;
        for i in (1..p.len()).step_by(2) {
            println!("{:?}, {:?}", i, p_chars[i]);
            if p_chars[i] != '*' {
                break;
            }


            dp[i+1][0] = true;
        }

        for row in 1..p_chars.len() + 1 {
            for col in 1..s_chars.len() + 1 {
                match p_chars[row - 1] {
                    '*' => {
                        println!("{:?} {:?}", row, col);
                        // If previous char does not match (take none)
                        if dp[row - 2][col] {
                            dp[row][col] = true;
                            continue;
                        }

                        // If previous char does match (take one to many)
                        let prev_char_matches_next = p_chars[row - 2] == s_chars[col - 1] || p_chars[row-2] == '.';
                        let prev_matched = dp[row][col -1];
                        if prev_char_matches_next && prev_matched {
                            dp[row][col] = true;
                        }
                    }
                    _ => {
                        println!(
                            "{:?}, {:?}, {:?}",
                            row,
                            col,
                            p_chars[row - 1] == s_chars[col - 1]
                        );

                        if (p_chars[row - 1] == s_chars[col - 1] || p_chars[row - 1] == '.') && dp[row - 1][col - 1]  {
                            dp[row][col] = true;
                        }
                    }
                }
            }
        }
        println!("{:?}", dp);
        dp[p_chars.len()][s_chars.len()]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_match("accb".to_owned(), "ac*b".to_owned()),
            true
        );
    }
}
