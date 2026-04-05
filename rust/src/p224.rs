impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<char> = vec![];
        let s_chars: Vec<char> = s.chars().collect();

        fn evaluate(mut chars: Vec<char>) -> Vec<char> {
            chars.reverse();
            let ret = vec![];
            let mut i = 0;
            while i < chars.len() {
                let curr_num: Vec<char> = Vec::new();
                while chars[i].is_digit(10) {
                    i += 1;
                }
            }

            ret
        }

        for i in 0..s_chars.len() {
            let c = s_chars[i];
            match c {
                ' ' => continue,
                ')' => {
                    // Evaluate until opening parentheses
                    let mut captured = vec![];
                    while stack.last().unwrap() != &'(' {
                        captured.push(stack.pop().unwrap());
                    }

                    // Get rid of (
                    stack.pop();

                    let mut evaluated = evaluate(captured);
                    stack.append(&mut evaluated);
                }
                _ => {
                    stack.push(c);
                }
            }
        }

        0
    }
}

struct Solution;
