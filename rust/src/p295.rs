use std::{cmp::Reverse, collections::BinaryHeap};

struct MedianFinder {
    larger_min_heap: BinaryHeap<Reverse<i32>>,
    smaller_max_heap: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            larger_min_heap: BinaryHeap::new(),
            smaller_max_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if let Some(max_top) = self.smaller_max_heap.peek()
            && num <= *max_top
        {
            self.smaller_max_heap.push(num);
        } else {
            self.larger_min_heap.push(Reverse(num));
        }
    }

    fn find_median(&mut self) -> f64 {
        while self.smaller_max_heap.len() > self.larger_min_heap.len() + 1 {
            self.larger_min_heap
                .push(Reverse(self.smaller_max_heap.pop().unwrap()));
        }
        while self.larger_min_heap.len() > self.smaller_max_heap.len() + 1 {
            self.smaller_max_heap
                .push(self.larger_min_heap.pop().unwrap().0);
        }

        if (self.larger_min_heap.len() + self.smaller_max_heap.len()) % 2 == 0 {
            let v1 = self.larger_min_heap.peek().unwrap_or(&Reverse(0)).0;
            let v2 = self.smaller_max_heap.peek().unwrap_or(&0);

            return (v1 as f64 + *v2 as f64) / 2.0;
        }

        if self.larger_min_heap.len() > self.smaller_max_heap.len() {
            return self.larger_min_heap.peek().unwrap_or(&Reverse(0)).0 as f64;
        }

        *self.smaller_max_heap.peek().unwrap_or(&0) as f64
    }
}
