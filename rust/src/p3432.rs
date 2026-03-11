pub fn count_partitions(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut left_sum = 0;
    let mut right_sum: i32 = nums.iter().sum();

    for i in 0..nums.len() - 1 {
        right_sum -= nums[i];
        left_sum += nums[i];
        if (right_sum - left_sum) % 2 == 0 {
            count += 1;
        }
    }
    return count;
}
