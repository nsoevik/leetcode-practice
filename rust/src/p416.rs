impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let s = nums.iter().sum::<i32>();
        if s % 2 == 1 {
            return false;
        }

        let end_target = (s / 2) as usize;

        let mut dp = vec![false; end_target + 1];
        dp[0] = true;

        // For every number in the set, we are checking every possible target.
        // If we can hit (target - number), we can hit the target.
        for num in nums {
            for target in ((num as usize)..=end_target).rev() {
                dp[target] |= dp[target - num as usize];
            }
        }

        dp[end_target]
    }
}

struct Solution;
