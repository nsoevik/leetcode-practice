use std::collections::HashMap;

struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store
            .entry(key)
            .or_insert(Vec::new())
            .push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let Some(values) = self.store.get(&key) else {
            return "".to_owned();
        };

        Self::binary(values, 0, values.len() - 1, timestamp).unwrap_or("".to_owned())
    }

    fn binary(values: &Vec<(i32, String)>, low: usize, high: usize, target: i32) -> Option<String> {
        if high < low {
            return None;
        }

        let mid = (low + high) / 2;
        let (ts, value) = &values[mid];
        if *ts > target {
            if mid == 0 {
                return None;
            }

            return Self::binary(values, low, mid - 1, target);
        }

        if mid + 1 >= values.len() || values[mid + 1].0 > target {
            return Some(value.clone());
        }

        Self::binary(values, mid+1, high, target)
    }
}
