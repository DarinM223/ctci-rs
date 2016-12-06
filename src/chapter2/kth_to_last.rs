use super::Node;

pub unsafe fn kth_to_last<T>(k: u32, node: *mut Node<T>) -> Option<*mut Node<T>> {
    let mut start = Some(node);
    let mut end = Some(node);

    for _ in 0..k {
        end.take().map(|node| end = (*node).next);
    }
    if end.is_none() {
        return None;
    }

    while let Some(node) = end {
        end = (*node).next;
        start.take().map(|node| start = (*node).next);
    }

    start
}

pub unsafe fn kth_to_last_rec<T>(k: u32,
                                 node: Option<*mut Node<T>>)
                                 -> (u32, Option<*mut Node<T>>) {
    if node.is_none() {
        return (0, None);
    }

    let (prev_count, prev_node) = kth_to_last_rec(k, node.and_then(|node| (*node).next));
    let count = prev_count + 1;
    if count == k {
        (count, node)
    } else {
        (count, prev_node)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use super::super::{compare_single_linked_list, free_single_linked_list, Node,
                       single_linked_list_from_vec};
    use super::*;

    unsafe fn compare_node_data<T>(case: i32, node: Option<*mut Node<T>>, value: Option<T>)
        where T: PartialEq + Debug
    {
        if value.is_none() {
            assert!(node.is_none());
            return;
        }

        let node = node.expect(format!("Error unwrapping node for case: {}", case).as_str());
        let value = value.expect(format!("Error unwrapping value for case: {}", case).as_str());

        assert_eq!((*node).data, value);
    }

    #[test]
    fn test_kth_to_last() {
        unsafe {
            let node = single_linked_list_from_vec(vec![1, 2, 1, 1, 3, 2, 5]);
            compare_single_linked_list(node, vec![1, 2, 1, 1, 3, 2, 5]);
            compare_node_data(0, kth_to_last(3, node), Some(3));
            compare_node_data(1, kth_to_last(2, node), Some(2));
            compare_node_data(2, kth_to_last(1, node), Some(5));
            compare_node_data(3, kth_to_last(0, node), None);
            compare_single_linked_list(node, vec![1, 2, 1, 1, 3, 2, 5]);
            free_single_linked_list(node);
        }
    }

    #[test]
    fn test_kth_to_last_rec() {
        unsafe {
            let node = single_linked_list_from_vec(vec![1, 2, 1, 1, 3, 2, 5]);
            compare_single_linked_list(node, vec![1, 2, 1, 1, 3, 2, 5]);
            compare_node_data(0, kth_to_last_rec(3, Some(node)).1, Some(3));
            compare_node_data(1, kth_to_last_rec(2, Some(node)).1, Some(2));
            compare_node_data(2, kth_to_last_rec(1, Some(node)).1, Some(5));
            compare_node_data(3, kth_to_last_rec(0, Some(node)).1, None);
            compare_single_linked_list(node, vec![1, 2, 1, 1, 3, 2, 5]);
            free_single_linked_list(node);
        }
    }
}
