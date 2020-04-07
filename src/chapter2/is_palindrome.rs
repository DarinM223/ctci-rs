use super::delete_middle_node::Node;

/// Uses a stack to save the linked list data so that popping from
/// the stack will give the reversed data one at a time.
pub fn is_palindrome<T>(node: &Option<Box<Node<T>>>) -> bool
where
    T: PartialEq + Clone,
{
    let mut stack = Vec::new();

    let mut curr = node.as_ref();
    while let Some(n) = curr.take() {
        stack.push(n.data.clone());
        curr = n.next.as_ref();
    }

    let mut curr = node.as_ref();
    while let Some(n) = curr.take() {
        let data = stack.pop().unwrap();
        if data != n.data {
            return false;
        }
        curr = n.next.as_ref();
    }

    true
}

#[cfg(test)]
mod tests {
    use super::super::delete_middle_node::list_from_vec;
    use super::is_palindrome;

    #[test]
    fn test_is_palindrome() {
        let node = list_from_vec(&mut vec![b'a', b'b', b'c', b'b', b'a']);
        assert_eq!(is_palindrome(&Some(node)), true);

        let node = list_from_vec(&mut vec![b'a', b'b', b'c', b'a', b'a']);
        assert_eq!(is_palindrome(&Some(node)), false);
    }
}
