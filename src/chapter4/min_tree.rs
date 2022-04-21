use super::Tree;

pub fn min_tree<T>(arr: &Vec<T>) -> Option<Box<Tree<T>>>
where
    T: Clone,
{
    min_tree_rec(arr, 0, arr.len() as i32 - 1)
}

/// Note: the start and end indexes must be integers because they can be negative.
fn min_tree_rec<T>(arr: &Vec<T>, start: i32, end: i32) -> Option<Box<Tree<T>>>
where
    T: Clone,
{
    if end < start {
        return None;
    }

    let mid = (start + end) / 2;
    let data = arr[mid as usize].clone();
    let left = min_tree_rec(&arr, start, mid - 1);
    let right = min_tree_rec(&arr, mid + 1, end);

    Some(Box::new(Tree {
        data: data,
        left: left,
        right: right,
    }))
}

#[cfg(test)]
mod tests {
    use super::super::tree_height;
    use super::*;

    #[test]
    fn test_min_tree() {
        let odd_list = vec![1, 2, 3, 6, 10];
        assert_eq!(tree_height(min_tree(&odd_list).as_deref()), 3);
    }
}
