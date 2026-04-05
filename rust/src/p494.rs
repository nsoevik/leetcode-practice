const MAX_SUM: i32 = 1000;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        // + or -
        //
        // x = ending index
        // y = target
        //
        //     1, 1, 2
        // -3
        // -2     1           // I can hit -2 with 1 -> (-3 target + 1 target)
        // -1  1
        // 0      2
        // 1   1
        // 2
        // 3

        fn to_target_index(i: &i32) -> Option<usize> {
            let sum = i + MAX_SUM;
            if sum < 0 || sum > 2 * MAX_SUM {
                return None;
            }
            return Some(sum as usize);
        }

        let mut dp = vec![vec![0; nums.len()]; (2 * MAX_SUM as usize) + 1];

        // Base (first number)
        let target_index_neg = to_target_index(&nums[0]).unwrap();
        let target_index_pos = to_target_index(&(-1 * nums[0])).unwrap();
        dp[target_index_neg][0] += 1;
        dp[target_index_pos][0] += 1;

        for vec_index in 1..nums.len() {
            for target in -MAX_SUM..=MAX_SUM {
                    let indexed_target = to_target_index(&target).unwrap();
                    if let Some(sum) = to_target_index(&(target - nums[vec_index])) {
                        dp[indexed_target][vec_index] += dp[sum][vec_index - 1].clone();
                    }
                    if let Some(sum) = to_target_index(&(target + nums[vec_index])) {
                        dp[indexed_target][vec_index] += dp[sum][vec_index - 1].clone();
                    }
                }
        }


        let indexed_res_target = to_target_index(&target).unwrap();
        dp[indexed_res_target][nums.len() - 1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
