use std::mem;

pub fn sort_stack<T>(stack: &mut Vec<T>)
where
    T: PartialOrd,
{
    let mut temp_stack = Vec::with_capacity(stack.len());
    while let Some(elem) = stack.pop() {
        while temp_stack.last().map(|last| elem > *last) == Some(true) {
            stack.push(temp_stack.pop().unwrap());
        }
        temp_stack.push(elem);
    }

    mem::swap(stack, &mut temp_stack);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_stack() {
        let mut stack = vec![5, 1, 3, 2, 6, 6, 1, 2];
        sort_stack(&mut stack);
        assert_eq!(stack, vec![6, 6, 5, 3, 2, 2, 1, 1]);
    }
}
