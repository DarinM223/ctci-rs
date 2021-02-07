use super::loop_detection::{Node, NodeKey};
use slotmap::SlotMap;

pub fn partition<T>(node: NodeKey, nodes: &mut SlotMap<NodeKey, Node<T>>, value: T) -> NodeKey
where
    T: PartialOrd,
{
    let mut head = node;
    let mut tail = node;
    let mut curr = node.next(nodes);

    while let Some(node) = curr {
        curr = node.next(nodes);
        let node_value = &mut nodes[node];
        if node_value.data < value {
            node_value.next = Some(head);
            head = node;
        } else {
            node_value.next = None;
            nodes[tail].next = Some(node);
            tail = node;
        }
    }

    head
}

#[cfg(test)]
mod tests {
    use super::super::loop_detection::{list_from_vec, vec_from_list};
    use super::*;

    #[test]
    fn test_partition() {
        let (mut slotmap, key) = list_from_vec(vec![3, 5, 8, 5, 10, 2, 1]);
        let result = partition(key.unwrap(), &mut slotmap, 5);
        assert_eq!(vec_from_list(result, &slotmap), vec![1, 2, 3, 5, 8, 5, 10]);
    }
}
