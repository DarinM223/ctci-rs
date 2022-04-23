pub mod bst_sequences;
pub mod build_order;
pub mod check_balanced;
pub mod check_subtree;
pub mod first_common_ancestor;
pub mod list_of_depths;
pub mod min_tree;
pub mod paths_with_sum;
pub mod random_node;
pub mod route_between_nodes;
pub mod successor;
pub mod validate_bst;

use std::cmp;
use std::fmt::Debug;
use std::{cell::Cell, ptr};
use typed_arena::Arena;

#[derive(PartialEq, Debug, Clone)]
pub struct Tree<T> {
    data: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

pub fn tree_height<T>(tree: Option<&Tree<T>>) -> usize {
    if tree.is_none() {
        return 0;
    }

    let left = tree_height(tree.and_then(|n| n.left.as_deref()));
    let right = tree_height(tree.and_then(|n| n.right.as_deref()));

    cmp::max(left, right) + 1
}

pub struct TreeNode<'a, T> {
    pub data: T,
    pub left: Cell<Option<&'a TreeNode<'a, T>>>,
    pub right: Cell<Option<&'a TreeNode<'a, T>>>,
    pub parent: Cell<Option<&'a TreeNode<'a, T>>>,
}

impl<'a, T> TreeNode<'a, T> {
    pub fn new(data: T, arena: &'a Arena<TreeNode<'a, T>>) -> &'a TreeNode<'a, T> {
        arena.alloc(TreeNode {
            data,
            left: Cell::new(None),
            right: Cell::new(None),
            parent: Cell::new(None),
        })
    }

    pub fn new_with_parent(
        data: T,
        parent: &'a TreeNode<'a, T>,
        left: bool,
        arena: &'a Arena<TreeNode<'a, T>>,
    ) -> &'a TreeNode<'a, T> {
        let node = arena.alloc(TreeNode {
            data,
            left: Cell::new(None),
            right: Cell::new(None),
            parent: Cell::new(Some(parent)),
        });
        if left {
            parent.left.set(Some(node));
        } else {
            parent.right.set(Some(node));
        }
        node
    }
}

impl<'a, T> Debug for &'a TreeNode<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TreeNode {{ data: {:?} }}", self.data)
    }
}

impl<'a, T> PartialEq for &'a TreeNode<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(*self, *other)
    }
}
