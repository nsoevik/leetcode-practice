use std::i32;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut chars = s.chars().peekable();
        // Ignore whitespace
        chars.next_if(|i| *i == ' ');

        // Determine signed
        let mut is_negative = false;
        chars.next_if(|c| *c == '-' || *c == '+').inspect(|c| {
            if *c == '-' {
                is_negative = true;
            }
        });

        // Round
        let mut captured = Vec::new();
        while let Some(digit) = chars.next_if(|c| c.is_digit(10)) {
            captured.push(digit);
        }

        println!("{:?}", captured);
        let mut number = captured.iter().collect::<String>().parse::<i64>().unwrap();
        if is_negative {
            number = number * -1;
        }

        if number > (i32::MAX as i64) {
            return i32::MAX;
        }

        if number < (i32::MIN as i64) {
            return i32::MIN;
        }

        return number as i32;
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
