pub struct MinStack<T> {
    stack: Vec<T>,
    min_stack: Vec<T>,
}

impl<T> MinStack<T>
where
    T: PartialOrd + Clone,
{
    pub fn new() -> MinStack<T> {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, data: T) {
        if let Some(last) = self.min_stack.last().cloned() {
            if data <= last {
                self.min_stack.push(data.clone());
            }
        } else {
            self.min_stack.push(data.clone());
        }
        self.stack.push(data);
    }

    pub fn pop(&mut self) -> Option<T> {
        let data = self.stack.pop();
        if !data.is_none() && data == self.min_stack.last().cloned() {
            self.min_stack.pop();
        }

        data
    }

    pub fn peek<'a>(&'a self) -> Option<&'a T> {
        self.stack.last()
    }

    pub fn min<'a>(&'a self) -> Option<&'a T> {
        self.min_stack.last()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_stack() {
        let mut stack = MinStack::new();
        stack.push(3);
        stack.push(1);
        stack.push(5);
        stack.push(2);
        stack.push(0);
        stack.push(0);
        stack.push(1);

        assert_eq!(stack.min(), Some(&0));
        stack.pop();
        assert_eq!(stack.min(), Some(&0));
        stack.pop();
        assert_eq!(stack.min(), Some(&0));
        stack.pop();
        assert_eq!(stack.min(), Some(&1));
        stack.pop();
        stack.pop();
        stack.pop();
        assert_eq!(stack.min(), Some(&3));
    }
}
