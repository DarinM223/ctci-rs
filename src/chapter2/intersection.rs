use std::ptr;

#[derive(Debug, Clone, Copy)]
pub struct Node<'a, T> {
    pub data: T,
    pub next: Option<&'a Node<'a, T>>,
}

pub type Ref<'a, T> = Option<&'a Node<'a, T>>;

impl<'a, T> PartialEq for &'a Node<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(*self, *other)
    }
}

pub fn intersection<'a, T>(mut l1: Ref<'a, T>, mut l2: Ref<'a, T>) -> Ref<'a, T> {
    let (l1_tail, len1) = get_tail_and_size(l1);
    let (l2_tail, len2) = get_tail_and_size(l2);
    if l1_tail != l2_tail {
        return None;
    }

    if len1 > len2 {
        for _ in 0..(len1 - len2) {
            l1 = l1.and_then(|n| n.next);
        }
    } else {
        for _ in 0..(len2 - len1) {
            l2 = l2.and_then(|n| n.next);
        }
    }

    while l1.is_some() && l2.is_some() {
        if l1 == l2 {
            return l1;
        }

        l1 = l1.and_then(|n| n.next);
        l2 = l2.and_then(|n| n.next);
    }

    None
}

fn get_tail_and_size<T>(l: Ref<'_, T>) -> (Ref<'_, T>, i32) {
    let mut curr = l;
    let mut len = 1;
    while curr.and_then(|n| n.next).is_some() {
        curr = curr.and_then(|n| n.next);
        len += 1;
    }
    (curr, len)
}

#[cfg(test)]
mod tests {
    use super::{intersection, Node};

    #[test]
    fn test_intersection() {
        // Diagram of linked list in test case:
        //
        //  3  --  8
        //
        //  5  --  9  --  7  --  2  --  1
        //               /
        //      6  --  4
        //
        let node1 = Node {
            data: 1,
            next: None,
        };
        let node2 = Node {
            data: 2,
            next: Some(&node1),
        };
        let node7 = Node {
            data: 7,
            next: Some(&node2),
        };
        let node9 = Node {
            data: 9,
            next: Some(&node7),
        };
        let node5 = Node {
            data: 5,
            next: Some(&node9),
        };
        let node4 = Node {
            data: 4,
            next: Some(&node7),
        };
        let node6 = Node {
            data: 6,
            next: Some(&node4),
        };
        let node8 = Node {
            data: 8,
            next: None,
        };
        let node3 = Node {
            data: 3,
            next: Some(&node8),
        };
        assert_eq!(intersection(Some(&node5), Some(&node6)), Some(&node7));
        assert_eq!(intersection(Some(&node6), Some(&node2)), Some(&node2));
        assert_eq!(intersection(Some(&node5), Some(&node3)), None);
    }
}
