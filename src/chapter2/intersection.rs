use super::Node;

pub unsafe fn intersection<T>(l1: *mut Node<T>, l2: *mut Node<T>) -> Option<*mut Node<T>> {
    let (l1_tail, len1) = get_tail_and_size(l1);
    let (l2_tail, len2) = get_tail_and_size(l2);

    if l1_tail != l2_tail {
        return None;
    }

    let mut node1 = Some(l1);
    let mut node2 = Some(l2);

    if len1 > len2 {
        for _ in 0..(len1 - len2) {
            node1 = node1.and_then(|n| (*n).next);
        }
    } else {
        for _ in 0..(len2 - len1) {
            node2 = node2.and_then(|n| (*n).next);
        }
    }

    while let (Some(n1), Some(n2)) = (node1, node2) {
        if n1 == n2 {
            return Some(n1);
        }

        node1 = (*n1).next;
        node2 = (*n2).next;
    }

    None
}

unsafe fn get_tail_and_size<T>(l: *mut Node<T>) -> (*mut Node<T>, i32) {
    let mut curr = Some(l);
    let mut len = 1;

    while !curr.and_then(|n| (*n).next).is_none() {
        curr = curr.and_then(|n| (*n).next);
        len += 1;
    }

    (curr.unwrap(), len)
}

#[cfg(test)]
mod tests {
    use super::super::{append_list, single_linked_list_from_vec};
    use super::*;

    #[test]
    fn test_intersection() {
        unsafe {
            let shared = single_linked_list_from_vec(vec![7, 2, 1]);
            let l1 = single_linked_list_from_vec(vec![3, 1, 5, 9]);
            let l2 = single_linked_list_from_vec(vec![4, 6]);
            let l3 = single_linked_list_from_vec(vec![4, 6, 7, 2, 1]);
            append_list(l1, shared);
            append_list(l2, shared);

            assert_eq!(intersection(l1, l2), Some(shared));
            assert_eq!(intersection(l1, l3), None);
        }
    }
}
