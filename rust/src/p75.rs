impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut low = 0;
        let mut mid = 0;
        let mut high = nums.len() - 1;

        while mid <= high {
            if nums[mid] == 0 {
                nums[mid] = nums[low];
                nums[low] = 0;
                mid += 1;
                low += 1;
            } else if nums[mid] == 2 {
                nums[mid] = nums[high];
                nums[high] = 2;
                if high == 0 {
                    break;
                }

                high -= 1;
            } else {
                mid += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut nums = vec![1, 0, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2])
    }
}

struct Solution;
