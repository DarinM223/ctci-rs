use super::{Tree, tree_height};
use std::collections::VecDeque;

pub fn list_of_depths<T>(tree: Tree<T>) -> Vec<Vec<Tree<T>>>
where
    T: Clone,
{
    let mut results = vec![vec![]; tree_height(Some(&tree))];
    let mut queue = VecDeque::new();
    queue.push_back((tree, 0));

    while let Some((mut node, level)) = queue.pop_front() {
        if let Some(left) = node.left.take() {
            queue.push_back((*left, level + 1));
        }
        if let Some(right) = node.right.take() {
            queue.push_back((*right, level + 1));
        }

        results[level].push(node);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::super::Tree;
    use super::*;

    #[test]
    fn test_list_of_depths() {
        let tree = Tree {
            data: 3,
            left: Some(Box::new(Tree {
                data: 4,
                left: Some(Box::new(Tree {
                    data: 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Tree {
                    data: 2,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(Tree {
                data: 6,
                left: Some(Box::new(Tree {
                    data: 8,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Tree {
                    data: 9,
                    left: None,
                    right: None,
                })),
            })),
        };

        assert_eq!(
            list_of_depths(tree),
            vec![
                vec![Tree {
                    data: 3,
                    left: None,
                    right: None,
                }],
                vec![
                    Tree {
                        data: 4,
                        left: None,
                        right: None,
                    },
                    Tree {
                        data: 6,
                        left: None,
                        right: None,
                    }
                ],
                vec![
                    Tree {
                        data: 1,
                        left: None,
                        right: None,
                    },
                    Tree {
                        data: 2,
                        left: None,
                        right: None,
                    },
                    Tree {
                        data: 8,
                        left: None,
                        right: None,
                    },
                    Tree {
                        data: 9,
                        left: None,
                        right: None,
                    }
                ]
            ]
        );
    }
}
