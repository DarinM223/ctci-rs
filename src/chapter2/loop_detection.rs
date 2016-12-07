use super::Node;

pub unsafe fn find_beginning<T>(l: *mut Node<T>) -> Option<*mut Node<T>> {
    let mut faster = (*l).next.and_then(|n| (*n).next);
    let mut slower = (*l).next;

    // Get intersection of the faster and slower nodes.
    while let (Some(s), Some(f)) = (slower.take(), faster.take()) {
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
    while let (Some(s), Some(i)) = (start.take(), intersection.take()) {
        if s == i {
            return Some(s);
        }

        start = (*s).next;
        intersection = (*i).next;
    }

    None
}

// TODO(DarinM223): add test cases and helper functions for find_beginning.
