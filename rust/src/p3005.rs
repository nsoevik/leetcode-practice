use std::collections::HashMap;

pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut m = HashMap::new();     
    for n in nums.iter() {
        *m.entry(n).or_insert(0) += 1;
    }

    let mut max_freq = 0;
    let mut ans = 0;
    for v in m.into_values() {
        if v > max_freq {
            max_freq = v;
            ans = 0;
            continue;
        }
        if v == max_freq {
            ans += 1;
        } 
    }
    return ans * max_freq;
}
