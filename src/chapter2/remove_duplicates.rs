use super::delete_middle_node::Node;
use std::collections::HashSet;
use std::hash::Hash;

pub fn remove_duplicates<T>(l: Option<Box<Node<T>>>) -> Option<Box<Node<T>>>
where
    T: Hash + Eq + Clone,
{
    remove_duplicates_rec(l, &mut HashSet::new())
}

fn remove_duplicates_rec<T: Hash + Eq + Clone>(
    l: Option<Box<Node<T>>>,
    set: &mut HashSet<T>,
) -> Option<Box<Node<T>>> {
    l.and_then(|mut node| {
        if !set.contains(&node.data) {
            set.insert(node.data.clone());
            node.next = remove_duplicates_rec(node.next, set);
            Some(node)
        } else {
            remove_duplicates_rec(node.next, set)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::super::delete_middle_node::{list_from_vec, vec_from_list};
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let list = list_from_vec(&mut vec![1, 2, 1, 1, 3, 2, 5]);
        let list_without_duplicates = remove_duplicates(Some(Box::new(list))).unwrap();
        assert_eq!(vec_from_list(&list_without_duplicates), vec![1, 2, 3, 5]);
    }
}
