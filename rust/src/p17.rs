const PHONE: [&str; 10] = [
    "0", "1", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ans = Vec::<String>::new();
        fn backtrack(ans: &mut Vec<String>, curr_output: String, input: &str) {
            if curr_output.len() == input.len() {
                ans.push(curr_output);
                return;
            } 

            let next_digit = &input[curr_output.len()..curr_output.len() + 1].parse::<usize>().unwrap();
            let selection = PHONE[*next_digit];

            for c in selection.chars() {
                let mut new_string = curr_output.clone();
                new_string.push(c);
                backtrack(ans, new_string, input);
            }
        }

        backtrack(&mut ans, "".to_string(), &digits);
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
