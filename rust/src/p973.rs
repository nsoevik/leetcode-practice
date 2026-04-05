use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        fn euclid_distance(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
            let x = p2[0] - p1[0];
            let y = p2[1] - p1[1];
            (x * x) + (y * y)
        }
        let mut h: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let zero = vec![0, 0];
        for i in 0..points.len() {
            let d = euclid_distance(&points[i], &zero);
            h.push((d, i));
        }

        while h.len() > k as usize {
            h.pop();
        }

        h.iter().map(|&(_, i)| points[i].clone()).collect()
    }
}

struct Solution;
