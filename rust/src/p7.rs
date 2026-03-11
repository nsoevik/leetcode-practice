use std::str::Chars;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut negative = false;
        if x < 0 {
            negative = true
        }

        let chars : Chars;
        let s = x.to_string();
        if negative {
            chars = s[1..].chars();
        } else {
            chars = s.chars();
        }

        let ans = chars.rev().collect::<String>().parse::<i32>().unwrap_or(0);
        if negative {
            return -1 * ans;
        }

        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}



// 12 -> 21
// 1200 -> 10101
