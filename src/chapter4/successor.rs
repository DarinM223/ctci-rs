use super::TreeNode;

pub unsafe fn inorder_successor<T>(tree: *mut TreeNode<T>) -> Option<*mut TreeNode<T>> {
    if let Some(mut node) = (*tree).right {
        while let Some(left) = (*node).left {
            node = left;
        }

        Some(node)
    } else {
        let mut node = tree;
        while let Some(parent) = (*node).parent {
            if (*parent).right == Some(node) {
                node = parent;
            } else {
                break;
            }
        }

        (*node).parent
    }
}

#[cfg(test)]
mod tests {
    use super::super::{free_tree, TreeNode};
    use super::*;

    #[test]
    fn test_inorder_successor_leftmost() {
        unsafe {
            let tree = TreeNode::new(8);
            let _ = TreeNode::new_with_parent(11, Some(tree), false);
            let three_node = TreeNode::new_with_parent(3, Some(tree), true);
            let _ = TreeNode::new_with_parent(1, Some(three_node), true);
            let four_node = TreeNode::new_with_parent(4, Some(three_node), false);
            let six_node = TreeNode::new_with_parent(6, Some(four_node), false);
            let five_node = TreeNode::new_with_parent(5, Some(six_node), true);
            let _ = TreeNode::new_with_parent(7, Some(six_node), false);

            assert_eq!(inorder_successor(four_node), Some(five_node));

            free_tree(tree);
        }
    }

    #[test]
    fn test_inorder_successor_parent() {
        unsafe {
            let tree = TreeNode::new(8);
            let _ = TreeNode::new_with_parent(11, Some(tree), false);
            let three_node = TreeNode::new_with_parent(3, Some(tree), true);
            let _ = TreeNode::new_with_parent(1, Some(three_node), true);
            let four_node = TreeNode::new_with_parent(4, Some(three_node), false);
            let five_node = TreeNode::new_with_parent(5, Some(four_node), false);

            assert_eq!(inorder_successor(five_node), Some(tree));
            free_tree(tree);
        }
    }
}
