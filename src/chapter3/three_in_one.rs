use std::result;

#[derive(Debug, PartialEq)]
pub enum StackError {
    StackFull,
    StackEmpty,
    InvalidStackIndex,
}

pub type Result<T> = result::Result<T, StackError>;

pub struct MultiStack<T> {
    array: Vec<Option<T>>,
    stack_capacity: usize,
    stack_sizes: [usize; 3],
}

impl<T> MultiStack<T>
where
    T: Clone,
{
    pub fn new(capacity: usize) -> MultiStack<T> {
        MultiStack {
            array: vec![None; capacity],
            stack_capacity: capacity / 3,
            stack_sizes: [0, 0, 0],
        }
    }

    pub fn push(&mut self, stack: usize, data: T) -> Result<()> {
        if stack > 2 {
            return Err(StackError::InvalidStackIndex);
        }
        if self.stack_sizes[stack] >= self.stack_capacity {
            return Err(StackError::StackFull);
        }

        let index = stack * self.stack_capacity + self.stack_sizes[stack];
        self.array[index] = Some(data);
        self.stack_sizes[stack] += 1;
        Ok(())
    }

    pub fn pop(&mut self, stack: usize) -> Result<T> {
        if stack > 2 {
            return Err(StackError::InvalidStackIndex);
        }
        if self.stack_sizes[stack] < 1 {
            return Err(StackError::StackEmpty);
        }

        let index = stack * self.stack_capacity + self.stack_sizes[stack] - 1;
        self.stack_sizes[stack] -= 1;

        Ok(self.array[index].take().unwrap())
    }

    pub fn peek<'a>(&'a mut self, stack: usize) -> Result<&'a T> {
        if stack > 2 {
            return Err(StackError::InvalidStackIndex);
        }
        if self.stack_sizes[stack] < 1 {
            return Err(StackError::StackEmpty);
        }

        let index = stack * self.stack_capacity + self.stack_sizes[stack] - 1;
        Ok(self.array[index].as_ref().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_stack() {
        let mut stack = MultiStack::new(6);
        assert_eq!(stack.push(0, 1), Ok(()));
        assert_eq!(stack.push(0, 2), Ok(()));
        assert_eq!(stack.push(0, 3), Err(StackError::StackFull));
        assert_eq!(stack.pop(0), Ok(2));
        assert_eq!(stack.pop(0), Ok(1));
        assert_eq!(stack.pop(0), Err(StackError::StackEmpty));

        assert_eq!(stack.push(1, 1), Ok(()));
        assert_eq!(stack.push(1, 2), Ok(()));
        assert_eq!(stack.push(1, 3), Err(StackError::StackFull));
        assert_eq!(stack.pop(1), Ok(2));
        assert_eq!(stack.pop(1), Ok(1));
        assert_eq!(stack.pop(1), Err(StackError::StackEmpty));

        assert_eq!(stack.push(2, 1), Ok(()));
        assert_eq!(stack.push(2, 2), Ok(()));
        assert_eq!(stack.push(2, 3), Err(StackError::StackFull));
        assert_eq!(stack.pop(2), Ok(2));
        assert_eq!(stack.pop(2), Ok(1));
        assert_eq!(stack.pop(2), Err(StackError::StackEmpty));

        assert_eq!(stack.push(3, 1), Err(StackError::InvalidStackIndex));
    }
}
