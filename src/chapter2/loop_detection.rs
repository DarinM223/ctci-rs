use slotmap::{new_key_type, SlotMap};

new_key_type! { pub struct NodeKey; }
pub struct Node<T> {
    pub data: T,
    pub next: Option<NodeKey>,
}

impl NodeKey {
    pub fn next<T>(self, nodes: &SlotMap<NodeKey, Node<T>>) -> Option<NodeKey> {
        nodes.get(self).and_then(|n| n.next)
    }
}

pub fn list_from_vec<T>(v: Vec<T>) -> (SlotMap<NodeKey, Node<T>>, Option<NodeKey>) {
    let mut slotmap = SlotMap::with_key();
    let mut prev = None;
    for v in v.into_iter().rev() {
        prev = Some(slotmap.insert(Node {
            data: v,
            next: prev,
        }));
    }
    (slotmap, prev)
}

pub fn vec_from_list<T: Clone>(node: NodeKey, nodes: &SlotMap<NodeKey, Node<T>>) -> Vec<T> {
    let mut vec = Vec::new();
    let mut curr_node = Some(node);
    while let Some(node) = curr_node {
        vec.push(nodes[node].data.clone());
        curr_node = node.next(nodes);
    }
    vec
}

pub fn find_beginning<T>(l: NodeKey, nodes: &SlotMap<NodeKey, Node<T>>) -> Option<NodeKey> {
    let mut faster = l.next(nodes).and_then(|n| n.next(nodes));
    let mut slower = l.next(nodes);

    // Get intersection of the faster and slower nodes.
    while let (Some(s), Some(f)) = (slower, faster) {
        if s == f {
            break;
        }

        slower = s.next(nodes);
        faster = f.next(nodes).and_then(|f| f.next(nodes));
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

        start = s.next(nodes);
        intersection = i.next(nodes);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_to_vec() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let (slotmap, key) = list_from_vec(v.clone());
        assert_eq!(vec_from_list(key.unwrap(), &slotmap), v);

        let v: Vec<i32> = Vec::new();
        let (_, key) = list_from_vec(v.clone());
        assert_eq!(key, None);
    }

    #[test]
    fn test_find_beginning() {
        let (mut nodes, node) = list_from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(find_beginning(node.unwrap(), &nodes), None);

        let mut eleventh = node;
        for _ in 0..10 {
            eleventh = eleventh.and_then(|n| n.next(&nodes));
        }
        assert_eq!(nodes[eleventh.unwrap()].data, 11);

        let mut fifth = node;
        for _ in 0..4 {
            fifth = fifth.and_then(|n| n.next(&nodes));
        }
        assert_eq!(nodes[fifth.unwrap()].data, 5);

        nodes[eleventh.unwrap()].next = fifth;
        assert_eq!(find_beginning(node.unwrap(), &nodes), fifth);
    }
}
