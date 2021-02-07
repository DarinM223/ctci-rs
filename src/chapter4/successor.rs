use super::first_common_ancestor::{TreeKey, TreeNode};
use slotmap::SlotMap;

pub fn inorder_successor<T>(
    tree: TreeKey,
    nodes: &SlotMap<TreeKey, TreeNode<T>>,
) -> Option<TreeKey> {
    if let Some(mut node) = tree.right(nodes) {
        while let Some(left) = node.left(nodes) {
            node = left;
        }

        Some(node)
    } else {
        let mut node = tree;
        while let Some(parent) = node.parent(nodes) {
            if parent.right(nodes) == Some(node) {
                node = parent;
            } else {
                break;
            }
        }

        node.parent(nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::super::first_common_ancestor::TreeKey;
    use super::*;

    #[test]
    fn test_inorder_successor_leftmost() {
        let mut nodes = SlotMap::with_key();
        let tree = TreeKey::new(8, &mut nodes);
        let _ = TreeKey::new_with_parent(11, tree, false, &mut nodes);
        let three_node = TreeKey::new_with_parent(3, tree, true, &mut nodes);
        let _ = TreeKey::new_with_parent(1, three_node, true, &mut nodes);
        let four_node = TreeKey::new_with_parent(4, three_node, false, &mut nodes);
        let six_node = TreeKey::new_with_parent(6, four_node, false, &mut nodes);
        let five_node = TreeKey::new_with_parent(5, six_node, true, &mut nodes);
        let _ = TreeKey::new_with_parent(7, six_node, false, &mut nodes);

        assert_eq!(inorder_successor(four_node, &nodes), Some(five_node));
    }

    #[test]
    fn test_inorder_successor_parent() {
        let mut nodes = SlotMap::with_key();
        let tree = TreeKey::new(8, &mut nodes);
        let _ = TreeKey::new_with_parent(11, tree, false, &mut nodes);
        let three_node = TreeKey::new_with_parent(3, tree, true, &mut nodes);
        let _ = TreeKey::new_with_parent(1, three_node, true, &mut nodes);
        let four_node = TreeKey::new_with_parent(4, three_node, false, &mut nodes);
        let five_node = TreeKey::new_with_parent(5, four_node, false, &mut nodes);

        assert_eq!(inorder_successor(five_node, &nodes), Some(tree));
    }
}
