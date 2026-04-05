impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut zipped = (0..start_time.len())
            .map(|i| (start_time[i], end_time[i], profit[i]))
            .collect::<Vec<(i32, i32, i32)>>();

        fn overlap(first: (i32, i32, i32), second: (i32, i32, i32)) -> bool {
            (first.0 > second.0 && first.0 < second.1)
                || (first.1 > second.0 && first.1 < second.1)
                || (second.0 > first.0 && second.0 < first.1)
                || (second.1 > first.0 && second.1 < first.1)
        }

        zipped.sort_by(|x,y| x.1.cmp(&y.1) );
        let mut dp = vec![0i32; start_time.len()];

        dp[0] = zipped[0].2;
        for i in 1..zipped.len() {
            let mut profit_if_taken = zipped[i].2;
            let mut profit_if_left = 0;

            // find next non-overlapping interval
            let mut low: i32 = 0;
            let mut high: i32 = i as i32 - 1;
            while low <= high {
                let mid = (low + (high - low) / 2) as usize;
                if !overlap(zipped[mid], zipped[i]) {
                    if mid + 1 == i {
                        profit_if_taken += dp[mid];
                        break;
                    }

                    if overlap(zipped[mid + 1], zipped[i]) {
                        profit_if_taken += dp[mid];
                        profit_if_left += dp[i - 1];
                        break;
                    }

                    low = mid as i32 + 1;
                    continue;
                }

                high = mid as i32 - 1;
            }

            dp[i] = profit_if_taken.max(profit_if_left);
        }

        dp[dp.len() - 1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let start_time = vec![1, 2, 3, 4, 6];
        let end_time = vec![3, 5, 10, 6, 9];
        let profit = vec![20, 20, 100, 70, 60];
        // Output: 150
        // Explanation: The subset chosen is the first, fourth and fifth job.
        // Profit obtained 150 = 20 + 70 + 60.
        //
        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 150);
    }
}
