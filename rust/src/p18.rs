#[derive(PartialEq, PartialOrd, Debug)]
struct Forwarder(usize);

impl Forwarder {
    fn forward(&mut self, nums: &Vec<i32>, step: isize) {
        let initial = self.0;
        let mut next = (initial as isize + step) as usize;

        while next < nums.len() && nums[initial] == nums[next] {
            let n = next as isize + step;
            if n < 0 { break; }
            next = n as usize;
        }

        self.0 = next;
    }
}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() - 3 {
            if i > 0 && nums[i] == nums[i - 1] { continue; }
            for j in i + 1..nums.len() - 2 {
                if nums[j] == nums[j - 1] { continue; }
                let mut low = Forwarder(j + 1);
                let mut high = Forwarder(nums.len() - 1);
                while low < high {
                    let s = nums[i] + nums[j] + nums[low.0] + nums[high.0];
                    match s {
                        s if s < target => low.forward(&nums, 1),
                        s if s > target => high.forward(&nums, -1),
                        _ => {
                            ans.push(vec![nums[i], nums[j], nums[low.0], nums[high.0]]);
                            low.forward(&nums, 1);
                            high.forward(&nums, -1);
                        }
                    }
                }
            }
        }

        ans
    }
}

struct Solution;
