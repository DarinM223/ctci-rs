/// The trick is to recursively get the subsets of the tail part
/// of the array and then clone the subsets and append the head to every cloned
/// subset and merge them with the normal subsets.
pub fn get_subsets(set: &Vec<i32>, index: usize) -> Vec<Vec<i32>> {
    if set.len() == index {
        vec![vec![]]
    } else {
        let mut all_subsets = get_subsets(set, index + 1);
        let item = *set.get(index).clone().unwrap();

        let mut more_subsets = all_subsets.clone();
        for subset in more_subsets.iter_mut() {
            subset.push(item);
        }

        all_subsets.append(&mut more_subsets);
        all_subsets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_subsets() {
        let set = vec![1, 2, 3];

        assert_eq!(
            get_subsets(&set, 0),
            vec![
                vec![],
                vec![3],
                vec![2],
                vec![3, 2],
                vec![1],
                vec![3, 1],
                vec![2, 1],
                vec![3, 2, 1]
            ]
        );
    }
}
