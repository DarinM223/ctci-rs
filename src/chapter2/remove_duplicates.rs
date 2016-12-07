use std::collections::HashMap;
use std::hash::Hash;
use std::mem;
use super::Node;

pub unsafe fn remove_duplicates<T>(l: *mut Node<T>)
    where T: Hash + Eq + Clone
{
    let mut prev = None;
    let mut curr = Some(l);
    let mut dict = HashMap::new();
    while let Some(node) = curr {
        curr = (*node).next;
        if !dict.contains_key(&(*node).data) {
            dict.insert((*node).data.clone(), true);
            prev = Some(node);
        } else {
            prev.map(move |prev| (*prev).next = curr);
            // Free node memory.
            mem::transmute::<*mut Node<T>, Box<Node<T>>>(node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{compare_single_linked_list, free_single_linked_list,
                       single_linked_list_from_vec};
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        unsafe {
            let node = single_linked_list_from_vec(vec![1, 2, 1, 1, 3, 2, 5]);
            compare_single_linked_list(node, vec![1, 2, 1, 1, 3, 2, 5]);
            remove_duplicates(node);
            compare_single_linked_list(node, vec![1, 2, 3, 5]);
            free_single_linked_list(node);
        }
    }
}
