impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut new_interval = new_interval;

        let mut i = 0;
        while i < intervals.len() && intervals[i][1] < new_interval[0] {
            ans.push(intervals[i].clone());
            i += 1;
        }

        while i < intervals.len() && intervals[i][0] <= new_interval[1] {
            new_interval[0] = new_interval[0].min(intervals[i][0]);
            new_interval[1] = new_interval[1].max(intervals[i][1]);
            i += 1;
        }

        ans.push(new_interval);
        while i < intervals.len() {
            ans.push(intervals[i].clone());
            i += 1;
        }

        ans
    }
}

struct Solution;
