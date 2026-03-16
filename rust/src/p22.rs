impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        fn backtrack(
            ans: &mut Vec<String>,
            mut curr: Vec<char>,
            right_used: i32,
            left_used: i32,
            n: i32,
        ) {
            if curr.len() == n * 2 {}
            if right_used == left_used {
                curr.push(')');
                continue;
            }
        }

        vec![]
    }
}
