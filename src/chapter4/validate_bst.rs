use super::Tree;

pub fn check_bst<T>(tree: Tree<T>) -> bool
    where T: PartialOrd + Copy
{
    check_bst_rec(Some(&Box::new(tree)), None, None)
}

fn check_bst_rec<T>(tree: Option<&Box<Tree<T>>>, start: Option<T>, end: Option<T>) -> bool
    where T: PartialOrd + Copy
{
    if let Some(t) = tree {
        let less_range = start.map_or(false, |d| t.data <= d);
        let greater_range = end.map_or(false, |d| t.data > d);

        if less_range || greater_range {
            false
        } else {
            check_bst_rec(t.left.as_ref(), start, Some(t.data.clone())) &&
            check_bst_rec(t.right.as_ref(), Some(t.data.clone()), end)
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
