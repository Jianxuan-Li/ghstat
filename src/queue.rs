use std::collections::VecDeque;

// a log queue, allow enqueue and dequeue log string
pub struct LogQueue {
    queue: VecDeque<String>,
}

impl LogQueue {
    pub fn new() -> Self {
        LogQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, log: String) {
        self.queue.push_back(log);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.queue.pop_front()
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }
}
