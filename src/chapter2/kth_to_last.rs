use super::intersection::Ref;

pub fn kth_to_last<'a, T>(k: u32, node: Ref<'a, T>) -> Ref<'a, T> {
    let mut start = node;
    let mut end = node;

    for _ in 0..k {
        end = end.and_then(|n| n.next);
    }
    if end.is_none() {
        return None;
    }

    while let Some(node) = end {
        end = node.next;
        start = start.and_then(|n| n.next);
    }

    start
}

pub fn kth_to_last_rec<'a, T>(k: u32, node: Ref<'a, T>) -> (u32, Ref<'a, T>) {
    if node.is_none() {
        return (0, None);
    }

    let (prev_count, prev_node) = kth_to_last_rec(k, node.and_then(|n| n.next));
    let count = prev_count + 1;
    if count == k {
        (count, node)
    } else {
        (count, prev_node)
    }
}

#[cfg(test)]
mod tests {
    use super::super::intersection::{Node, Ref};
    use super::{kth_to_last, kth_to_last_rec};
    use crate::ref_list;

    #[test]
    fn test_kth_to_last() {
        let node = ref_list!(1, 2, 1, 1, 3, 2, 5);
        assert_eq!(kth_to_last(3, node).map(|n| n.data), Some(3));
        assert_eq!(kth_to_last(2, node).map(|n| n.data), Some(2));
        assert_eq!(kth_to_last(1, node).map(|n| n.data), Some(5));
        assert_eq!(kth_to_last(0, node).map(|n| n.data), None);
        let empty: Ref<i32> = None;
        assert_eq!(kth_to_last(1, empty).map(|n| n.data), None);
    }

    #[test]
    fn test_kth_to_last_rec() {
        let node = ref_list!(1, 2, 1, 1, 3, 2, 5);
        assert_eq!(kth_to_last_rec(3, node).1.map(|n| n.data), Some(3));
        assert_eq!(kth_to_last_rec(2, node).1.map(|n| n.data), Some(2));
        assert_eq!(kth_to_last_rec(1, node).1.map(|n| n.data), Some(5));
        assert_eq!(kth_to_last_rec(0, node).1.map(|n| n.data), None);
        let empty: Ref<i32> = None;
        assert_eq!(kth_to_last_rec(1, empty).1.map(|n| n.data), None);
    }
}
