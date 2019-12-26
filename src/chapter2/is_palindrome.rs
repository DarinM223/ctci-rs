use super::Node;

/// Uses a stack to save the linked list data so that popping from
/// the stack will give the reversed data one at a time.
pub unsafe fn is_palindrome<T>(node: *mut Node<T>) -> bool
where
    T: PartialEq + Clone,
{
    let mut stack = Vec::new();

    let mut curr = Some(node);
    while let Some(n) = curr {
        stack.push((*n).data.clone());
        curr = (*n).next;
    }

    let mut curr = Some(node);
    while let Some(n) = curr {
        let data = stack.pop().unwrap();
        if data != (*n).data {
            return false;
        }
        curr = (*n).next;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::super::{
        compare_single_linked_list, free_single_linked_list, single_linked_list_from_vec,
    };
    use super::*;

    #[test]
    fn test_is_palindrome() {
        unsafe {
            let node = single_linked_list_from_vec(vec![b'a', b'b', b'c', b'b', b'a']);
            assert_eq!(is_palindrome(node), true);
            compare_single_linked_list(node, vec![b'a', b'b', b'c', b'b', b'a']);

            let node = single_linked_list_from_vec(vec![b'a', b'b', b'c', b'a', b'a']);
            assert_eq!(is_palindrome(node), false);
            compare_single_linked_list(node, vec![b'a', b'b', b'c', b'a', b'a']);

            free_single_linked_list(node);
        }
    }
}
