use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut map = HashMap::<char, i32>::new();
        for task in tasks {
            *map.entry(task).or_insert(0) += 1;
        }

        let mut h = BinaryHeap::from_iter(map.iter().map(|(_, v)| *v));
        let mut total_time = 0;
        while !h.is_empty() {
            let mut tmp_heap = BinaryHeap::<i32>::new();
            let mut used = 0;
            while used < n + 1 && !h.is_empty() {
                used += 1;
                let todo = h.pop().unwrap();
                if todo > 1 {
                    tmp_heap.push(todo - 1);
                }
            }

            while let Some(v) = h.pop() {
                tmp_heap.push(v);
            }
            if tmp_heap.is_empty() {
                total_time += used;
                break;
            }

            h = tmp_heap;
            total_time += n + 1;
        }
        total_time
    }
}

struct Solution;
