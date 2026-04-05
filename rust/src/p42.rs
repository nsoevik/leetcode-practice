impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut ans = 0;
        for i in 0..height.len() {
            println!("Before {:?}", stack);
            let mut last_height_used: Option<i32> = None;
            while let Some((_, prev_height)) = stack.last() {
                if height[i] < *prev_height {
                    break;
                }

                let (prev_index, prev_height) = stack.pop().unwrap();
                // Area (x*y) of filled area
                let x = i - prev_index - 1;
                let mut y = prev_height.min(height[i]);
                if let Some(last_height) = last_height_used {
                    y -= last_height;
                }

                ans += x as i32 * y;
                last_height_used = Some(prev_height);
            }

            stack.push((i, height[i]));
            println!("After {:?} {:?}", stack, ans);
        }

        ans
    }
}

struct Solution;
