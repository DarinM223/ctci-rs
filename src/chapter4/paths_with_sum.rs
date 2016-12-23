use std::collections::HashMap;
use super::Tree;

/// Brute force O(nlogn) recursive algorithm that counts the number of paths
/// starting from the root node, then recursively calls itself on its left and right child
/// and adds the paths together.
pub fn paths_with_sum(tree: Option<&Box<Tree<i32>>>, target_sum: i32) -> i32 {
    if tree.is_none() {
        return 0;
    }

    let paths = paths_from_node(tree, target_sum, 0);
    let left_paths = paths_with_sum(tree.and_then(|n| n.left.as_ref()), target_sum);
    let right_paths = paths_with_sum(tree.and_then(|n| n.right.as_ref()), target_sum);

    paths + left_paths + right_paths
}

fn paths_from_node(tree: Option<&Box<Tree<i32>>>, target_sum: i32, current_sum: i32) -> i32 {
    if let Some(node) = tree {
        let current_sum = current_sum + node.data;

        let mut total_paths = 0;
        if current_sum == target_sum {
            total_paths += 1;
        }

        total_paths += paths_from_node(node.left.as_ref(), target_sum, current_sum);
        total_paths += paths_from_node(node.right.as_ref(), target_sum, current_sum);

        total_paths
    } else {
        0
    }
}

/// O(n) time, O(log n) space solution.
pub fn paths_with_sum_opt(tree: Option<&Box<Tree<i32>>>, target_sum: i32) -> i32 {
    paths_with_sum_opt_rec(tree, target_sum, 0, &mut HashMap::new())
}

fn paths_with_sum_opt_rec(tree: Option<&Box<Tree<i32>>>,
                          target_sum: i32,
                          running_sum: i32,
                          path_count: &mut HashMap<i32, i32>)
                          -> i32 {
    if let Some(node) = tree {
        let running_sum = running_sum + node.data;
        let sum = running_sum - target_sum;

        let mut paths = path_count.get(&sum).cloned().unwrap_or(0);
        if running_sum == target_sum {
            paths += 1;
        }

        increment_hash_table(path_count, running_sum, 1);
        paths += paths_with_sum_opt_rec(node.left.as_ref(), target_sum, running_sum, path_count);
        paths += paths_with_sum_opt_rec(node.right.as_ref(), target_sum, running_sum, path_count);
        increment_hash_table(path_count, running_sum, -1);

        paths
    } else {
        0
    }
}

fn increment_hash_table(table: &mut HashMap<i32, i32>, key: i32, delta: i32) {
    let new_count = table.get(&key).cloned().unwrap_or(0) + delta;
    if new_count == 0 {
        table.remove(&key);
    } else {
        table.insert(key, new_count);
    }
}

#[cfg(test)]
mod tests {
    use super::super::Tree;
    use super::*;

    #[test]
    fn test_paths_with_sum() {
        let tree = Box::new(Tree {
            data: 10,
            left: Some(Box::new(Tree {
                data: 5,
                left: Some(Box::new(Tree {
                    data: 3,
                    left: Some(Box::new(Tree {
                        data: 3,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Tree {
                        data: -2,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(Tree {
                    data: 2,
                    left: None,
                    right: Some(Box::new(Tree {
                        data: 1,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            right: Some(Box::new(Tree {
                data: -3,
                left: None,
                right: Some(Box::new(Tree {
                    data: 11,
                    left: None,
                    right: None,
                })),
            })),
        });

        assert_eq!(paths_with_sum(Some(&tree), 8),
                   paths_with_sum_opt(Some(&tree), 8));
    }
}
