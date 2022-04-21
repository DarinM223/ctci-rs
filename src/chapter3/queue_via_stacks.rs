use std::mem;

pub struct StackQueue<T> {
    stack_newest: Vec<T>,
    stack_oldest: Vec<T>,
}

impl<T> StackQueue<T> {
    pub fn new() -> StackQueue<T> {
        StackQueue {
            stack_newest: Vec::new(),
            stack_oldest: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, data: T) {
        self.stack_newest.push(data);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.move_newest_to_oldest();
        self.stack_oldest.pop()
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.move_newest_to_oldest();
        self.stack_oldest.last()
    }

    pub fn len(&self) -> usize {
        self.stack_newest.len() + self.stack_oldest.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack_newest.is_empty() && self.stack_oldest.is_empty()
    }

    fn move_newest_to_oldest(&mut self) {
        if self.stack_oldest.is_empty() {
            self.stack_newest.reverse();
            mem::swap(&mut self.stack_newest, &mut self.stack_oldest);
        }
    }
}

impl<T> Default for StackQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_queue() {
        let mut queue = StackQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.is_empty(), true);
    }
}
