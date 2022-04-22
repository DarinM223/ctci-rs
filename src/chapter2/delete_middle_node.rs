#[derive(Clone, Debug, PartialEq)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub fn list_from_vec<T>(v: &mut Vec<T>) -> Node<T> {
    let mut node = Node {
        data: v.pop().unwrap(),
        next: None,
    };
    while let Some(data) = v.pop() {
        node = Node {
            data,
            next: Some(Box::new(node)),
        };
    }

    node
}

pub fn vec_from_list<T: Clone>(n: &Node<T>) -> Vec<T> {
    let mut vec = Vec::new();
    fn vec_from_list_iter<T: Clone>(n: &Node<T>, vec: &mut Vec<T>) {
        vec.push(n.data.clone());
        if let Some(next) = n.next.as_deref() {
            vec_from_list_iter(next, vec);
        }
    }

    vec_from_list_iter(n, &mut vec);
    vec
}

pub fn delete_middle_node<T>(node: &mut Node<T>) {
    if let Some(next) = node.next.take() {
        node.data = next.data;
        node.next = next.next;
    }
}

#[cfg(test)]
mod tests {
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
