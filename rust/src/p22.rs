impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        fn backtrack(
            ans: &mut Vec<String>,
            mut curr: Vec<char>,
            right_used: usize,
            left_used: usize,
            n: usize,
        ) {
            if curr.len() == n * 2 {
                ans.push(curr.iter().collect());
                return;
            }

            if right_used < left_used {
                let mut c = curr.clone();
                c.push(')');
                backtrack(ans, c, right_used + 1, left_used, n);
            }

            if left_used < n {
                curr.push('(');
                backtrack(ans, curr, right_used, left_used + 1, n);
            }
        }

        backtrack(&mut ans, vec![], 0, 0, n as usize);
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::generate_parenthesis(3), Vec::<String>::new());
    }
}
