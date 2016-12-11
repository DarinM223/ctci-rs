use std::cmp;
use std::result;
use super::Tree;

pub fn is_balanced<T>(root: Tree<T>) -> bool {
    check_height(Some(&Box::new(root))).is_ok()
}

struct InvalidHeight;
type Result<T> = result::Result<T, InvalidHeight>;

fn check_height<T>(node: Option<&Box<Tree<T>>>) -> Result<i32> {
    if node.is_none() {
        return Ok(-1);
    }

    let left_height = check_height(node.and_then(|n| n.left.as_ref()))?;
    let right_height = check_height(node.and_then(|n| n.right.as_ref()))?;

    if (left_height - right_height).abs() > 1 {
        Err(InvalidHeight)
    } else {
        Ok(cmp::max(left_height, right_height) + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Tree;
    use super::*;

    #[test]
    fn test_is_balanced() {
        let unbalanced_tree = Tree {
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
                    right: Some(Box::new(Tree {
                        data: 5,
                        left: None,
                        right: None,
                    })),
                })),
                right: None,
            })),
        };
        assert_eq!(is_balanced(unbalanced_tree), false);

        let balanced_tree = Tree {
            data: 1,
            left: Some(Box::new(Tree {
                data: 2,
                left: None,
                right: Some(Box::new(Tree {
                    data: 5,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(Tree {
                data: 3,
                left: Some(Box::new(Tree {
                    data: 4,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
        };
        assert_eq!(is_balanced(balanced_tree), true);
    }
}
