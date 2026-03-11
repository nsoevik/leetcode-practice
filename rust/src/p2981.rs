use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut h: HashMap<(char, usize), u32> = HashMap::new();
        let mut curr_char = '-';
        let mut count_of_chars: u32 = 0;
        let mut chars = s.chars().peekable();
        while let Some(char) = chars.next() {
            if curr_char != char {
                curr_char = char;
                count_of_chars = 1;
            } else {
                count_of_chars += 1;
            }

            let peek = chars.peek();
            if peek.is_none_or(|c| *c != char) {
                for len in 1..=count_of_chars {
                    *h.entry((char, len as usize)).or_insert(0) += 1 * count_of_chars;
                    count_of_chars -= 1;
                }
            }
        }

        let mut max_so_far = 0;
        for (k, v) in h.iter() {
            if *v >= 3 {
                max_so_far = max_so_far.max(k.1);
            }
        }

        let mut ans = max_so_far as i32;
        if ans == 0 {
            ans = -1
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::maximum_length("tessetesssesss".to_string()), 2)
    }

    #[test]
    fn basic2() {
        assert_eq!(Solution::maximum_length("tesssetesssesss".to_string()), 3)
    }

    #[test]
    fn basic3() {
        assert_eq!(Solution::maximum_length("aaaa".to_string()), 2)
    }
}
