impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let h_len = height.len();
        if h_len < 2 {
            return 0;
        }

        let mut ans = 0;
        let mut front = 0;
        let mut back = h_len - 1;
        while front < back {
            let min_height = height[front].min(height[back]);
            ans = ans.max((back - front) as i32 * min_height);
            if height[front] < height[back] {
                front += 1;
                continue;
            }

            back -= 1;
            continue;
        }

        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
