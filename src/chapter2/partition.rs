use super::Node;

pub unsafe fn partition<T>(node: *mut Node<T>, value: T) -> *mut Node<T>
    where T: PartialOrd
{
    let mut head = node;
    let mut tail = node;
    let mut curr = (*node).next;

    while let Some(n) = curr.take() {
        curr = (*n).next;
        if (*n).data < value {
            (*n).next = Some(head);
            head = n;
        } else {
            (*n).next = None;
            (*tail).next = Some(n);
            tail = n;
        }
    }

    head
}

#[cfg(test)]
mod tests {
    use super::super::{compare_single_linked_list, free_single_linked_list,
                       single_linked_list_from_vec};
    use super::*;

    #[test]
    fn test_partition() {
        unsafe {
            let node = single_linked_list_from_vec(vec![3, 5, 8, 5, 10, 2, 1]);
            let node = partition(node, 5);
            compare_single_linked_list(node, vec![1, 2, 3, 5, 8, 5, 10]);
            free_single_linked_list(node);
        }
    }
}
