struct Entry {
    val: i32,
    min_so_far: i32,
}

struct MinStack {
    entries: Vec<Entry>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let last = self.entries.last();
        let mut min_so_far: i32 = val;
        if let Some(entry) = last {
            min_so_far = val.min(entry.min_so_far);
        } 

        self.entries.push(Entry { val, min_so_far });
    }

    fn pop(&mut self) {
        self.entries.pop();
    }

    fn top(&self) -> i32 {
        if let Some(entry) = self.entries.last() {
            return entry.val;
        } 
        
        0
    }

    fn get_min(&self) -> i32 {
        if let Some(entry) = self.entries.last() {
            return entry.min_so_far;
        }

        0
    }
}
