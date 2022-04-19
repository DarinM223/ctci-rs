use super::loop_detection::Node;

pub fn partition<'a, T>(node: &'a Node<'a, T>, value: T) -> &'a Node<'a, T>
where
    T: PartialOrd,
{
    let mut head = node;
    let mut tail = node;
    let mut curr = node.next.get();

    while let Some(node) = curr {
        curr = node.next.get();
        if node.data < value {
            node.next.set(Some(head));
            head = node;
        } else {
            node.next.set(None);
            tail.next.set(Some(node));
            tail = node;
        }
    }

    head
}

#[cfg(test)]
mod tests {
    use super::super::loop_detection::{list_from_vec, vec_from_list};
    use super::*;
    use typed_arena::Arena;

    #[test]
    fn test_partition() {
        let arena = Arena::new();
        let node = list_from_vec(vec![3, 5, 8, 5, 10, 2, 1], &arena);
        let result = partition(node.unwrap(), 5);
        assert_eq!(vec_from_list(result), vec![1, 2, 3, 5, 8, 5, 10]);
    }
}
