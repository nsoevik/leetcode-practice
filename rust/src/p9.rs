impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let chars: Vec<char> = x.to_string().chars().collect();
        let char_len = chars.len();
        for i in 0..char_len/2 {
            if chars[i] != chars[char_len - i - 1] {
                return false;
            }
        }

        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_palindrome(12321), true);
        assert_eq!(Solution::is_palindrome(12331), false);
    }
}
