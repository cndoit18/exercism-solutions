use std::collections::{BTreeMap, BinaryHeap};

pub struct School(BTreeMap<u32, BinaryHeap<String>>);

impl School {
    pub fn new() -> School {
        School(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self
            .0
            .values()
            .any(|heap| heap.iter().any(|name| name == student))
        {
            return;
        }
        self.0.entry(grade).or_default().push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.0
            .get(&grade)
            .cloned()
            .unwrap_or(BinaryHeap::new())
            .into_sorted_vec()
    }
}
