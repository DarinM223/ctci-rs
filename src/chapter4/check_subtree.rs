use super::Tree;
use std::fmt::{Debug, Error, Write};

pub fn contains_subtree<T>(tree: Option<&Tree<T>>, subtree: Option<&Tree<T>>) -> bool
where
    T: PartialEq,
{
    // If the subtree is None, there is nothing to look for.
    if subtree.is_none() {
        return true;
    }

    check_subtree(tree, subtree)
}

/// Recursive function that checks if one of the branches is a subtree through the
/// is_subtree function. If it isn't then it goes down the left and right children
/// while keeping the subtree the same.
fn check_subtree<T>(tree: Option<&Tree<T>>, subtree: Option<&Tree<T>>) -> bool
where
    T: PartialEq,
{
    // If the bigger tree is None and we are still looking for the subtree,
    // the subtree is not inside the bigger tree.
    if tree.is_none() {
        return false;
    } else if is_subtree(tree, subtree) {
        return true;
    }

    check_subtree(tree.and_then(|n| n.left.as_deref()), subtree)
        || check_subtree(tree.and_then(|n| n.right.as_deref()), subtree)
}

/// Recursive function that checks if the two trees are identical. It returns
/// false if the two node's data are not the same. Otherwise, it recursively checks
/// the left children of both nodes and the right children of both nodes to see if they are
/// identical.
fn is_subtree<T>(tree: Option<&Tree<T>>, subtree: Option<&Tree<T>>) -> bool
where
    T: PartialEq,
{
    // If both the tree and the subtree are None that branch is a match.
    // Otherwise, fail if the nodes are different.
    if tree.is_none() && subtree.is_none() {
        return true;
    } else if tree.map(|n| &n.data) != subtree.map(|n| &n.data) {
        return false;
    }

    is_subtree(
        tree.and_then(|n| n.left.as_deref()),
        subtree.and_then(|n| n.left.as_deref()),
    ) && is_subtree(
        tree.and_then(|n| n.right.as_deref()),
        subtree.and_then(|n| n.right.as_deref()),
    )
}

/// The simple way is to create preorder string representations of both trees and then
/// check if the subtree string is a substring of the main tree string.
pub fn contains_subtree_simple<T>(
    tree: Option<&Tree<T>>,
    subtree: Option<&Tree<T>>,
) -> Result<bool, Error>
where
    T: Debug,
{
    let mut s1 = String::new();
    let mut s2 = String::new();

    subtree_str(tree, &mut s1)?;
    subtree_str(subtree, &mut s2)?;

    Ok(s1.contains(s2.as_str()))
}

/// Writes the string representation of the preorder traversal of the tree.
fn subtree_str<T>(node: Option<&Tree<T>>, builder: &mut String) -> Result<(), Error>
where
    T: Debug,
{
    if let Some(n) = node {
        write!(builder, "{:?} ", n.data)?;

        subtree_str(node.and_then(|n| n.left.as_deref()), builder)?;
        subtree_str(node.and_then(|n| n.right.as_deref()), builder)?;
    } else {
        write!(builder, "X")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::Tree;
    use super::*;

    #[test]
    fn test_contains_normal() {
        let tree = Tree {
            data: 1,
            left: Some(Box::new(Tree {
                data: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Tree {
                data: 3,
                left: Some(Box::new(Tree {
                    data: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Tree {
                    data: 5,
                    left: None,
                    right: None,
                })),
            })),
        };

        let test_tree = Tree {
            data: 3,
            left: Some(Box::new(Tree {
                data: 4,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Tree {
                data: 5,
                left: None,
                right: None,
            })),
        };

        assert_eq!(contains_subtree(Some(&tree), Some(&test_tree)), true);
        assert_eq!(
            contains_subtree_simple(Some(&tree), Some(&test_tree)),
            Ok(true)
        );
    }

    #[test]
    fn test_contains_subtree_different_structures() {
        let tree = Tree {
            data: 3,
            left: Some(Box::new(Tree {
                data: 4,
                left: None,
                right: None,
            })),
            right: None,
        };

        let test_tree = Tree {
            data: 3,
            left: None,
            right: Some(Box::new(Tree {
                data: 5,
                left: None,
                right: None,
            })),
        };

        assert_eq!(contains_subtree(Some(&tree), Some(&test_tree)), false);
        assert_eq!(
            contains_subtree_simple(Some(&tree), Some(&test_tree)),
            Ok(false)
        );
    }
}
