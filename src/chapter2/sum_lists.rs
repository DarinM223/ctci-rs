use super::{Node, single_linked_list_len};

/// This function is easiest to implement recursively.
/// The main difficulty is safely handling different sized lists.
pub unsafe fn sum_lists(l1: Option<*mut Node<i32>>,
                        l2: Option<*mut Node<i32>>,
                        carry: i32)
                        -> Option<*mut Node<i32>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }

    let l1_val = l1.map(|n| (*n).data).unwrap_or(0);
    let l2_val = l2.map(|n| (*n).data).unwrap_or(0);
    let result = l1_val + l2_val + carry;
    let digit = result % 10;
    let new_carry = if result >= 10 {
        1
    } else {
        0
    };

    let node = Node::new(digit);
    if l1.is_some() || l2.is_some() {
        let next_node = sum_lists(l1.and_then(|n| (*n).next),
                                  l2.and_then(|n| (*n).next),
                                  new_carry);
        (*node).next = next_node;
    }

    Some(node)
}

/// When adding in reverse you need a helper function to return both the sum list node
/// and the carry. Also the lists must be padded to be the same length.
pub unsafe fn sum_lists_rev(l1: *mut Node<i32>, l2: *mut Node<i32>) -> Option<*mut Node<i32>> {
    let l1_len = single_linked_list_len(l1);
    let l2_len = single_linked_list_len(l2);

    let (mut l1, mut l2) = (l1, l2);
    if l1_len < l2_len {
        l1 = pad_list(l1, l2_len - l1_len);
    } else {
        l2 = pad_list(l2, l1_len - l2_len);
    }

    let (node, carry) = sum_lists_rev_helper(Some(l1), Some(l2));
    if carry == 0 {
        node
    } else {
        let new_head = Node::new(carry);
        (*new_head).next = node;
        Some(new_head)
    }
}

/// Helper function that sums the lists assuming that the lists lengths are the same.
unsafe fn sum_lists_rev_helper(l1: Option<*mut Node<i32>>,
                               l2: Option<*mut Node<i32>>)
                               -> (Option<*mut Node<i32>>, i32) {
    if l1.is_none() && l2.is_none() {
        return (None, 0);
    }

    let (prev_node, prev_carry) = sum_lists_rev_helper(l1.and_then(|n| (*n).next),
                                                       l2.and_then(|n| (*n).next));
    let l1_val = l1.map(|n| (*n).data).unwrap_or(0);
    let l2_val = l2.map(|n| (*n).data).unwrap_or(0);
    let result = l1_val + l2_val + prev_carry;
    let digit = result % 10;
    let carry = if result >= 10 {
        1
    } else {
        0
    };
    let node = Node::new(digit);
    (*node).next = prev_node;

    (Some(node), carry)
}

/// Pad list with a certain amount of zeros.
unsafe fn pad_list(l: *mut Node<i32>, padding: i32) -> *mut Node<i32> {
    let mut head = l;
    for _ in 0..padding {
        let node = Node::new(0);
        (*node).next = Some(head);
        head = node;
    }
    head
}

#[cfg(test)]
mod tests {
    use super::super::{compare_single_linked_list, free_single_linked_list,
                       single_linked_list_from_vec};
    use super::*;

    #[test]
    fn test_basic_sum_lists() {
        unsafe {
            let l1 = single_linked_list_from_vec(vec![7, 1, 6]);
            let l2 = single_linked_list_from_vec(vec![5, 9, 2]);

            let result = sum_lists(Some(l1), Some(l2), 0).unwrap();
            compare_single_linked_list(result, vec![2, 1, 9]);

            free_single_linked_list(l1);
            free_single_linked_list(l2);
            free_single_linked_list(result);
        }
    }

    #[test]
    fn test_different_sizes_sum_lists() {
        unsafe {
            let l1 = single_linked_list_from_vec(vec![2]);
            let l2 = single_linked_list_from_vec(vec![9, 9, 9]);

            let result = sum_lists(Some(l1), Some(l2), 0).unwrap();
            compare_single_linked_list(result, vec![1, 0, 0, 1]);

            free_single_linked_list(l1);
            free_single_linked_list(l2);
            free_single_linked_list(result);
        }
    }

    #[test]
    fn test_basic_sum_lists_rev() {
        unsafe {
            let l1 = single_linked_list_from_vec(vec![6, 1, 7]);
            let l2 = single_linked_list_from_vec(vec![2, 9, 5]);

            let result = sum_lists_rev(l1, l2).unwrap();
            compare_single_linked_list(result, vec![9, 1, 2]);

            free_single_linked_list(l1);
            free_single_linked_list(l2);
            free_single_linked_list(result);
        }
    }

    #[test]
    fn test_different_sizes_sum_lists_rev() {
        unsafe {
            let l1 = single_linked_list_from_vec(vec![2]);
            let l2 = single_linked_list_from_vec(vec![9, 9, 9]);

            let result = sum_lists_rev(l1, l2).unwrap();
            compare_single_linked_list(result, vec![1, 0, 0, 1]);

            free_single_linked_list(l1);
            free_single_linked_list(l2);
            free_single_linked_list(result);
        }
    }
}
