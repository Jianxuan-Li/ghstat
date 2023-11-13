use crate::fileutil;

pub struct Counter {
    count: i32,
    prev_count: i32,
}

impl Counter {
    pub fn new(counter_file: &str) -> Self {
        Counter {
            count: fileutil::read_count(counter_file).unwrap_or(0),
            prev_count: -1,
        }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn get_count(&self) -> i32 {
        self.count
    }

    pub fn get_prev_count(&self) -> i32 {
        self.prev_count
    }

    pub fn set_prev_count(&mut self, prev_count: i32) {
        self.prev_count = prev_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let file_name = "counter_test.txt";
        fileutil::create_file(file_name, "15");
        let mut counter = Counter::new(file_name);
        assert_eq!(counter.get_count(), 15);
        counter.increment();
        assert_eq!(counter.get_count(), 16);
    }
}
