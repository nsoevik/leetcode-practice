struct EntryList {
    values: Vec<i32>,
}

impl EntryList {
    fn add(&mut self, val: i32) {
        self.values.push(val);
    }
    fn del(&mut self, key: i32) {
        self.values.push(val);
    }
}

struct MyHashMap {
    arr: Vec<i32>,
    entries_used: usize,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            entries_used: 0,
            arr: Vec::new(),
        }
    }

    fn hash(&self, key: i32) -> i32 {
        key.wrapping_mul(320192) % self.arr.len() as i32
    }

    fn put(&self, key: i32, value: i32) {
        let hash = self.hash(key);

    }

    fn get(&self, key: i32) -> i32 {}

    fn put(&mut self, key: i32, value: i32) {}

    fn remove(&self, key: i32) {}
}
