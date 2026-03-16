use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let m = HashMap::<char,char>::from([
            ('[', ']'),
            ('{', '}'),
            ('(', ')'),
        ]); 

        let mut stack: Vec<char> = vec![];
        let chars = s.chars();
        for c in chars {
            println!("{:?}", stack);
            if stack.is_empty() {
                stack.push(c);
                continue;
            }

            let rhs = m.get(&stack[stack.len() - 1]);
            match rhs {
                Some(right_side) => {
                    if *right_side == c {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                },
                None => return false,
            }
        }

        stack.is_empty()
    }
}

struct Solution;
