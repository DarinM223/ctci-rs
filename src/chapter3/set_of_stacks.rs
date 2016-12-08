use std::collections::VecDeque;

pub struct SetOfStacks<T> {
    stacks: Vec<VecDeque<T>>,
    capacity: usize,
}

impl<T> SetOfStacks<T> {
    pub fn new(capacity: usize) -> SetOfStacks<T> {
        SetOfStacks {
            stacks: Vec::new(),
            capacity: capacity,
        }
    }

    pub fn push(&mut self, data: T) {
        if self.stacks.is_empty() || self.stacks.last().map(|s| s.len()).unwrap() >= self.capacity {
            self.stacks.push(VecDeque::with_capacity(self.capacity));
        }

        self.stacks.last_mut().map(move |s| s.push_back(data));
    }

    pub fn pop(&mut self) -> Option<T> {
        let data = self.stacks.last_mut().and_then(|s| s.pop_back());
        if self.stacks.last().map(|s| s.is_empty()) == Some(true) {
            self.stacks.pop();
        }
        data
    }

    pub fn pop_at(&mut self, index: usize) -> Option<T> {
        let data = self.stacks.get_mut(index).and_then(|s| s.pop_back());
        for i in (index + 1)..self.stacks.len() {
            let front = self.stacks[i].pop_front().unwrap();
            self.stacks[i - 1].push_back(front);
        }
        if self.stacks.last().map(|s| s.is_empty()) == Some(true) {
            self.stacks.pop();
        }
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_of_stacks() {
        // Test with 1 element stack.
        let mut stack = SetOfStacks::new(1);
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Pop from second stack.
        assert_eq!(stack.pop_at(1), Some(2));

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(1));

        stack.push(1);
        assert_eq!(stack.pop_at(0), Some(1));
        assert_eq!(stack.pop_at(0), None);

        // Test with 2 element stack.
        let mut stack = SetOfStacks::new(2);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);

        assert_eq!(stack.pop_at(0), Some(2));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(1));
    }
}
