use super::TreeNode;

pub fn inorder_successor<'a, T>(tree: &'a TreeNode<'a, T>) -> Option<&'a TreeNode<'a, T>> {
    if let Some(mut node) = tree.right.get() {
        while let Some(left) = node.left.get() {
            node = left;
        }

        Some(node)
    } else {
        let mut node = tree;
        while let Some(parent) = node.parent.get() {
            if parent.right.get() == Some(node) {
                node = parent;
            } else {
                break;
            }
        }

        node.parent.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typed_arena::Arena;

    #[test]
    fn test_inorder_successor_leftmost() {
        let arena = Arena::new();
        let tree = TreeNode::new(8, &arena);
        let _ = TreeNode::new_with_parent(11, tree, false, &arena);
        let three_node = TreeNode::new_with_parent(3, tree, true, &arena);
        let _ = TreeNode::new_with_parent(1, three_node, true, &arena);
        let four_node = TreeNode::new_with_parent(4, three_node, false, &arena);
        let six_node = TreeNode::new_with_parent(6, four_node, false, &arena);
        let five_node = TreeNode::new_with_parent(5, six_node, true, &arena);
        let _ = TreeNode::new_with_parent(7, six_node, false, &arena);

        assert_eq!(inorder_successor(four_node), Some(five_node));
    }

    #[test]
    fn test_inorder_successor_parent() {
        let arena = Arena::new();
        let tree = TreeNode::new(8, &arena);
        let _ = TreeNode::new_with_parent(11, tree, false, &arena);
        let three_node = TreeNode::new_with_parent(3, tree, true, &arena);
        let _ = TreeNode::new_with_parent(1, three_node, true, &arena);
        let four_node = TreeNode::new_with_parent(4, three_node, false, &arena);
        let five_node = TreeNode::new_with_parent(5, four_node, false, &arena);

        assert_eq!(inorder_successor(five_node), Some(tree));
    }
}
