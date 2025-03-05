use super::Tree;

pub fn check_bst<T>(tree: Tree<T>) -> bool
where
    T: PartialOrd + Copy,
{
    check_bst_rec(Some(&tree), None, None)
}

fn check_bst_rec<T>(tree: Option<&Tree<T>>, start: Option<T>, end: Option<T>) -> bool
where
    T: PartialOrd + Copy,
{
    if let Some(t) = tree {
        let less_range = start.is_some_and(|d| t.data <= d);
        let greater_range = end.is_some_and(|d| t.data > d);

        if less_range || greater_range {
            false
        } else {
            check_bst_rec(t.left.as_deref(), start, Some(t.data))
                && check_bst_rec(t.right.as_deref(), Some(t.data), end)
        }
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::super::Tree;
    use super::*;

    #[test]
    fn test_check_bst() {
        let invalid_tree = Tree {
            data: 20,
            left: Some(Box::new(Tree {
                data: 10,
                left: None,
                right: Some(Box::new(Tree {
                    data: 25,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(Tree {
                data: 30,
                left: None,
                right: None,
            })),
        };

        assert_eq!(check_bst(invalid_tree), false);

        let valid_tree = Tree {
            data: 20,
            left: Some(Box::new(Tree {
                data: 10,
                left: Some(Box::new(Tree {
                    data: 5,
                    left: Some(Box::new(Tree {
                        data: 3,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Tree {
                        data: 7,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(Tree {
                    data: 15,
                    left: None,
                    right: Some(Box::new(Tree {
                        data: 17,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            right: Some(Box::new(Tree {
                data: 30,
                left: None,
                right: None,
            })),
        };

        assert_eq!(check_bst(valid_tree), true);
    }
}
