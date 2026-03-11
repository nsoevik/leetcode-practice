impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s
        }
        let mut rows: Vec<Vec<char>> = vec![vec![]; num_rows as usize];
        let mut going_down = true;

        let mut curr_index = 0;
        for c in s.chars() {
            rows[curr_index].push(c);
            if (going_down && curr_index >= (num_rows-1) as usize) || (!going_down && curr_index == 0) {
                going_down = !going_down;
            }

            if going_down {
                curr_index += 1;
                continue;
            }

            curr_index -= 1;
        }

        rows.iter().map(|vec| vec.iter().collect::<String>()).collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
