impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        // -4, -1, 1, 2 (1)
        let mut ans = -1;
        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() - 2 {
            let mut low = i + 1;
            let mut high = nums.len() - 1;
            while low < high {
                let s = nums[i] + nums[low] + nums[high];
                let prev_diff = (target - ans).abs();
                let new_diff = (target - s).abs();
                if new_diff < prev_diff || ans == -1 {
                    ans = s
                }
                
                match s {
                    s if s == target => {
                        return s;
                    }
                    s if s > target => high -= 1,
                    _ => low += 1,
                }
            }
        }

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
