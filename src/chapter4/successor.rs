use std::mem;

/// A tree node that also contains a pointer to the parent node.
pub struct Node<T> {
    data: T,
    left: Option<*mut Node<T>>,
    right: Option<*mut Node<T>>,
    parent: Option<*mut Node<T>>,
}

impl<T> Node<T> {
    pub unsafe fn new(data: T) -> *mut Node<T> {
        let node = Box::new(Node {
            data: data,
            left: None,
            right: None,
            parent: None,
        });

        mem::transmute::<Box<Node<T>>, *mut Node<T>>(node)
    }

    pub unsafe fn new_with_parent(data: T,
                                  parent: Option<*mut Node<T>>,
                                  left: bool)
                                  -> *mut Node<T> {
        let node = Box::new(Node {
            data: data,
            left: None,
            right: None,
            parent: parent,
        });
        let node_ptr = mem::transmute::<Box<Node<T>>, *mut Node<T>>(node);

        // Link parent's left or right pointer to the node.
        parent.map(|p| {
            if left {
                (*p).left = Some(node_ptr);
            } else {
                (*p).right = Some(node_ptr);
            }
        });

        node_ptr
    }
}

pub unsafe fn free_tree<T>(tree: *mut Node<T>) {
    if let Some(left) = (*tree).left {
        free_tree(left);
    }
    if let Some(right) = (*tree).right {
        free_tree(right);
    }

    // Free the node.
    mem::transmute::<*mut Node<T>, Box<Node<T>>>(tree);
}

pub unsafe fn inorder_successor<T>(tree: *mut Node<T>) -> Option<*mut Node<T>> {
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
    use super::*;

    #[test]
    fn test_inorder_successor_leftmost() {
        unsafe {
            let tree = Node::new(8);
            let eleven_node = Node::new_with_parent(11, Some(tree), false);
            let three_node = Node::new_with_parent(3, Some(tree), true);
            let one_node = Node::new_with_parent(1, Some(three_node), true);
            let four_node = Node::new_with_parent(4, Some(three_node), false);
            let six_node = Node::new_with_parent(6, Some(four_node), false);
            let five_node = Node::new_with_parent(5, Some(six_node), true);
            let seven_node = Node::new_with_parent(7, Some(six_node), false);

            assert_eq!(inorder_successor(four_node), Some(five_node));

            free_tree(tree);
        }
    }

    #[test]
    fn test_inorder_successor_parent() {
        unsafe {
            let tree = Node::new(8);
            let eleven_node = Node::new_with_parent(11, Some(tree), false);
            let three_node = Node::new_with_parent(3, Some(tree), true);
            let one_node = Node::new_with_parent(1, Some(three_node), true);
            let four_node = Node::new_with_parent(4, Some(three_node), false);
            let five_node = Node::new_with_parent(5, Some(four_node), false);

            assert_eq!(inorder_successor(five_node), Some(tree));
            free_tree(tree);
        }
    }
}
