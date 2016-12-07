use super::Node;

pub unsafe fn find_beginning<T>(l: *mut Node<T>) -> Option<*mut Node<T>> {
    let mut faster = (*l).next.and_then(|n| (*n).next);
    let mut slower = (*l).next;

    // Get intersection of the faster and slower nodes.
    while let (Some(s), Some(f)) = (slower, faster) {
        if s == f {
            break;
        }

        slower = (*s).next;
        faster = (*f).next.and_then(|n| (*n).next);
    }

    // Return early if not loop.
    if faster.is_none() {
        return None;
    }

    // Get intersection starting from the intersected node and the head of the list.
    let mut start = Some(l);
    let mut intersection = faster;
    while let (Some(s), Some(i)) = (start, intersection) {
        if s == i {
            return Some(s);
        }

        start = (*s).next;
        intersection = (*i).next;
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::super::{append_list, single_linked_list_from_vec};
    use super::*;

    #[test]
    fn test_find_beginning() {
        unsafe {
            let node = single_linked_list_from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
            assert_eq!(find_beginning(node), None);

            let mut fourth = Some(node);
            for _ in 0..3 {
                fourth = fourth.and_then(|n| (*n).next);
            }
            append_list(node, fourth.unwrap());

            assert_eq!(find_beginning(node), fourth);
        }
    }
}
