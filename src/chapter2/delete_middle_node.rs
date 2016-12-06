use std::mem;
use super::Node;

pub unsafe fn delete_middle_node<T>(node: *mut Node<T>)
    where T: Clone
{
    let mut curr = Some(node);
    let mut next = (*node).next;

    while let Some(next_node) = next.take() {
        curr.map(move |curr_node| {
            (*curr_node).data = (*next_node).data.clone();
        });

        if (*next_node).next.is_none() {
            curr.map(|curr_node| (*curr_node).next = None);
            // Free the last node.
            mem::transmute::<*mut Node<T>, Box<Node<T>>>(next_node);
            break;
        }

        curr = Some(next_node);
        next = (*next_node).next;
    }
}

#[cfg(test)]
mod tests {
    use super::super::{compare_single_linked_list, free_single_linked_list,
                       single_linked_list_from_vec};
    use super::*;

    #[test]
    fn test_delete_middle_node() {
        unsafe {
            let node = single_linked_list_from_vec(vec![1, 2, 3, 4, 5, 6]);
            let second_node = (*node).next.unwrap();
            delete_middle_node(second_node);
            compare_single_linked_list(node, vec![1, 3, 4, 5, 6]);

            let second_to_last_node = (*(*(*node).next.unwrap()).next.unwrap()).next.unwrap();
            delete_middle_node(second_to_last_node);
            compare_single_linked_list(node, vec![1, 3, 4, 6]);

            free_single_linked_list(node);
        }
    }
}
