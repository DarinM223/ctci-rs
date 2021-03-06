#[derive(Debug, Clone, Copy)]
pub struct Node<'a, T> {
    pub data: T,
    pub next: Option<&'a Node<'a, T>>,
}

pub type Ref<'a, T> = Option<&'a Node<'a, T>>;

#[macro_export]
macro_rules! ref_list {
    ($x:expr) => {{
        Some(&Node {
            data: $x,
            next: None,
        })
    }};
    ($x:expr, $($rest:tt)*) => {{
        Some(&Node {
            data: $x,
            next: ref_list!($($rest)*),
        })
    }};
}

pub fn as_ptr<'a, T>(r: Ref<'a, T>) -> Option<*const Node<'a, T>> {
    r.map(|n| n as *const Node<'a, T>)
}

pub fn intersection<'a, T>(mut l1: Ref<'a, T>, mut l2: Ref<'a, T>) -> Ref<'a, T> {
    let (l1_tail, len1) = get_tail_and_size(l1);
    let (l2_tail, len2) = get_tail_and_size(l2);
    if as_ptr(l1_tail) != as_ptr(l2_tail) {
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
        if as_ptr(l1) == as_ptr(l2) {
            return l1;
        }

        l1 = l1.and_then(|n| n.next);
        l2 = l2.and_then(|n| n.next);
    }

    None
}

fn get_tail_and_size<'a, T>(l: Ref<'a, T>) -> (Ref<'a, T>, i32) {
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
    use super::{as_ptr, intersection, Node};

    #[test]
    fn test_intersection() {
        let node1 = Node {
            data: 1,
            next: None,
        };
        let node2 = Node {
            data: 2,
            next: Some(&node1),
        };
        let node3 = Node {
            data: 7,
            next: Some(&node2),
        };

        let node4 = Node {
            data: 9,
            next: Some(&node3),
        };
        let node5 = Node {
            data: 5,
            next: Some(&node4),
        };

        let node6 = Node {
            data: 4,
            next: Some(&node3),
        };
        let node7 = Node {
            data: 6,
            next: Some(&node6),
        };

        let node8 = Node {
            data: 1,
            next: None,
        };
        let node9 = Node {
            data: 2,
            next: Some(&node8),
        };

        assert_eq!(
            as_ptr(intersection(Some(&node5), Some(&node7))),
            as_ptr(Some(&node3))
        );
        assert_eq!(
            as_ptr(intersection(Some(&node7), Some(&node2))),
            as_ptr(Some(&node2))
        );
        assert_eq!(as_ptr(intersection(Some(&node5), Some(&node9))), None);
    }
}
