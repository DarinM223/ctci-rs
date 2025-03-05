use super::Node;

pub fn kth_to_last<T>(k: u32, node: Option<&Node<T>>) -> Option<&Node<T>> {
    let mut start = node;
    let mut end = node;

    for _ in 0..k {
        end = end.and_then(|n| n.next.as_deref());
    }
    end?;

    while let Some(node) = end {
        end = node.next.as_deref();
        start = start.and_then(|n| n.next.as_deref());
    }

    start
}

pub fn kth_to_last_rec<T>(k: u32, node: Option<&Node<T>>) -> (u32, Option<&Node<T>>) {
    if node.is_none() {
        return (0, None);
    }

    let (prev_count, prev_node) = kth_to_last_rec(k, node.and_then(|n| n.next.as_deref()));
    let count = prev_count + 1;
    if count == k {
        (count, node)
    } else {
        (count, prev_node)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Node, list_from_vec};
    use super::{kth_to_last, kth_to_last_rec};

    #[test]
    fn test_kth_to_last() {
        let node = list_from_vec(&mut vec![1, 2, 1, 1, 3, 2, 5]);
        assert_eq!(kth_to_last(3, Some(&node)).map(|n| n.data), Some(3));
        assert_eq!(kth_to_last(2, Some(&node)).map(|n| n.data), Some(2));
        assert_eq!(kth_to_last(1, Some(&node)).map(|n| n.data), Some(5));
        assert_eq!(kth_to_last(0, Some(&node)).map(|n| n.data), None);
        let empty: Option<&Node<i32>> = None;
        assert_eq!(kth_to_last(1, empty).map(|n| n.data), None);
    }

    #[test]
    fn test_kth_to_last_rec() {
        let node = list_from_vec(&mut vec![1, 2, 1, 1, 3, 2, 5]);
        assert_eq!(kth_to_last_rec(3, Some(&node)).1.map(|n| n.data), Some(3));
        assert_eq!(kth_to_last_rec(2, Some(&node)).1.map(|n| n.data), Some(2));
        assert_eq!(kth_to_last_rec(1, Some(&node)).1.map(|n| n.data), Some(5));
        assert_eq!(kth_to_last_rec(0, Some(&node)).1.map(|n| n.data), None);
        let empty: Option<&Node<i32>> = None;
        assert_eq!(kth_to_last_rec(1, empty).1.map(|n| n.data), None);
    }
}
