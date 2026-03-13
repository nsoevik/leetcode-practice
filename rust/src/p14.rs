use std::collections::{HashMap, HashSet, hash_map::Entry};

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: HashSet<Vec<i32>> = HashSet::new();
        let mut h = HashMap::<i32, usize>::new();
        for i in 0..nums.len() {
            h.insert(-nums[i], i);
        }

        for i in 0..nums.len() - 1 {
            for j in i+1..nums.len() {
                let s = nums[i] + nums[j];
                if let Entry::Occupied(e) = h.entry(s) {
                    let entry_index = e.get();
                    if i == *entry_index || j == *entry_index {
                        continue;
                    }

                    let mut v = vec![nums[i], nums[j], nums[*entry_index]];
                    v.sort();
                    ans.insert(v);
                }
            }
        }

        ans.into_iter().collect::<Vec<Vec<i32>>>()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
}
