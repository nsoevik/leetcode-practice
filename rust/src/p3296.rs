struct Solution;

use std::{cmp::Ordering, collections::BinaryHeap};
#[derive(PartialEq, Eq, Debug)]
struct UsageTracker {
    next_time_done: i64,
    worker_time: i64,
    counts_used: i64,
}

impl Ord for UsageTracker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if other.next_time_done > self.next_time_done {
            return Ordering::Greater;
        } else if other.next_time_done == self.next_time_done {
            return Ordering::Equal;
        } else {
            return Ordering::Less;
        }
    }
}

impl PartialOrd for UsageTracker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut heap = BinaryHeap::from(
            worker_times
                .iter()
                .map(|t| UsageTracker {
                    next_time_done: *t as i64,
                    counts_used: 0,
                    worker_time: *t as i64,
                })
                .collect::<Vec<UsageTracker>>(),
        );

        let mut height_to_go = mountain_height;
        let mut time_used = 0;
        while height_to_go > 0 {
            let mut next_usage = heap.pop().unwrap();
            println!("used {:?}",next_usage);
            height_to_go -= 1;
            time_used = next_usage.next_time_done;
            next_usage.counts_used += 1;
            next_usage.next_time_done += next_usage.worker_time * (next_usage.counts_used + 1);
            heap.push(next_usage);
        }

        time_used as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_number_of_seconds(2, vec![1, 2, 3]), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::min_number_of_seconds(3, vec![1, 2, 3]), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_number_of_seconds(3, vec![5]), 30);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::min_number_of_seconds(4, vec![2, 1, 1]), 3);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::min_number_of_seconds(100000, vec![1]), 5000050000);
    }
}
