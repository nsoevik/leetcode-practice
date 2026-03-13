impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = "".to_string();
        if strs.len() == 0 {
            return prefix;
        }

        let first_chars: Vec<char> = strs[0].clone().chars().collect();
        for char in first_chars {
            prefix.push(char); 

            for str in strs.iter() {
                if !str.starts_with(&prefix) {
                    prefix.pop();
                    return prefix;
                }
            }
        }

        prefix
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl".to_string());
    }
}
