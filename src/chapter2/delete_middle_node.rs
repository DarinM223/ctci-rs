use super::Node;

pub fn delete_middle_node<T>(node: &mut Node<T>) {
    if let Some(next) = node.next.take() {
        node.data = next.data;
        node.next = next.next;
    }
}

#[cfg(test)]
mod tests {
    use super::super::{list_from_vec, vec_from_list};
    use super::*;

    #[test]
    fn test_delete_middle_node() {
        let mut node = list_from_vec(&mut vec![1, 2, 3, 4, 5, 6]);
        if let Some(second_node) = node.next.as_deref_mut() {
            delete_middle_node(second_node);
        }
        assert_eq!(vec_from_list(&node), vec![1, 3, 4, 5, 6]);

        if let Some(second_to_last_node) = node
            .next
            .as_deref_mut()
            .and_then(|n| n.next.as_deref_mut())
            .and_then(|n| n.next.as_deref_mut())
        {
            delete_middle_node(second_to_last_node);
        }
        assert_eq!(vec_from_list(&node), vec![1, 3, 4, 6]);
    }
}
