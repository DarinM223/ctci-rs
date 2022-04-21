use super::delete_middle_node::Node;

/// This function is easiest to implement recursively.
/// The main difficulty is safely handling different sized lists.
pub fn sum_lists(
    l1: Option<Box<Node<i32>>>,
    l2: Option<Box<Node<i32>>>,
    carry: i32,
) -> Option<Box<Node<i32>>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }

    let l1_val = l1.as_deref().map(|n| n.data).unwrap_or(0);
    let l2_val = l2.as_deref().map(|n| n.data).unwrap_or(0);
    let result = l1_val + l2_val + carry;
    let digit = result % 10;
    let new_carry = if result >= 10 { 1 } else { 0 };

    let next = if l1.is_some() || l2.is_some() {
        sum_lists(l1.and_then(|n| n.next), l2.and_then(|n| n.next), new_carry)
    } else {
        None
    };

    Some(Box::new(Node { data: digit, next }))
}

/// When adding in reverse you need a helper function to return both the sum list node
/// and the carry. Also the lists must be padded to be the same length.
pub fn sum_lists_rev(
    l1: Option<Box<Node<i32>>>,
    l2: Option<Box<Node<i32>>>,
) -> Option<Box<Node<i32>>> {
    let l1_len = list_len(&l1);
    let l2_len = list_len(&l2);

    let (padded_l1, padded_l2) = if l1_len < l2_len {
        (pad_list(l1, l2_len - l1_len), l2)
    } else {
        (l1, pad_list(l2, l1_len - l2_len))
    };

    let (node, carry) = sum_lists_rev_helper(padded_l1, padded_l2);
    if carry == 0 {
        node
    } else {
        Some(Box::new(Node {
            data: carry,
            next: node,
        }))
    }
}

/// Returns the length of the list
fn list_len(l: &Option<Box<Node<i32>>>) -> usize {
    let mut len = 0;
    let mut head = l.as_deref();
    while let Some(node) = head {
        len += 1;
        head = node.next.as_deref();
    }
    len
}

/// Helper function that sums the lists assuming that the lists lengths are the same.
fn sum_lists_rev_helper(
    l1: Option<Box<Node<i32>>>,
    l2: Option<Box<Node<i32>>>,
) -> (Option<Box<Node<i32>>>, i32) {
    if l1.is_none() && l2.is_none() {
        return (None, 0);
    }

    let l1_val = l1.as_deref().map(|n| n.data).unwrap_or(0);
    let l2_val = l2.as_deref().map(|n| n.data).unwrap_or(0);
    let (prev_node, prev_carry) =
        sum_lists_rev_helper(l1.and_then(|n| n.next), l2.and_then(|n| n.next));
    let result = l1_val + l2_val + prev_carry;
    let digit = result % 10;
    let carry = if result >= 10 { 1 } else { 0 };
    let node = Some(Box::new(Node {
        data: digit,
        next: prev_node,
    }));
    (node, carry)
}

/// Pads list with a certain amount of zeros.
fn pad_list(l: Option<Box<Node<i32>>>, padding: usize) -> Option<Box<Node<i32>>> {
    let mut head = l;
    for _ in 0..padding {
        head = Some(Box::new(Node {
            data: 0,
            next: head,
        }));
    }
    head
}

#[cfg(test)]
mod tests {
    use super::super::delete_middle_node::{list_from_vec, vec_from_list};
    use super::*;

    #[test]
    fn test_basic_sum_lists() {
        let l1 = Some(Box::new(list_from_vec(&mut vec![7, 1, 6])));
        let l2 = Some(Box::new(list_from_vec(&mut vec![5, 9, 2])));
        let sum = sum_lists(l1, l2, 0).unwrap();
        assert_eq!(vec_from_list(&sum), vec![2, 1, 9]);
    }

    #[test]
    fn test_different_sizes_sum_lists() {
        let l1 = Some(Box::new(list_from_vec(&mut vec![2])));
        let l2 = Some(Box::new(list_from_vec(&mut vec![9, 9, 9])));
        let sum = sum_lists(l1, l2, 0).unwrap();
        assert_eq!(vec_from_list(&sum), vec![1, 0, 0, 1]);
    }

    #[test]
    fn test_basic_sum_lists_rev() {
        let l1 = Some(Box::new(list_from_vec(&mut vec![6, 1, 7])));
        let l2 = Some(Box::new(list_from_vec(&mut vec![2, 9, 5])));
        let sum = sum_lists_rev(l1, l2).unwrap();
        assert_eq!(vec_from_list(&sum), vec![9, 1, 2]);
    }

    #[test]
    fn test_different_sizes_sum_lists_rev() {
        let l1 = Some(Box::new(list_from_vec(&mut vec![2])));
        let l2 = Some(Box::new(list_from_vec(&mut vec![9, 9, 9])));
        let sum = sum_lists_rev(l1, l2).unwrap();
        assert_eq!(vec_from_list(&sum), vec![1, 0, 0, 1]);
    }
}
