impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n as usize]; m as usize];
        dp[0][0] = 1;
        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                }
                let mut above = 0;
                let mut left = 0;

                if let Some(row) = dp.get(i-1) {
                    above = row[j];
                }
                if let Some(val) = dp[i].get(j-1) {
                    left = *val;
                }

                dp[i][j] = above + left;
            }
        }

        dp[m - 1][n - 1]
    }
}

struct Solution;
