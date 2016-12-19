use std::collections::VecDeque;
use super::Tree;

pub fn all_sequences<T>(node: Option<&Box<Tree<T>>>) -> Vec<VecDeque<T>>
    where T: Clone
{
    let mut result = Vec::new();

    if node.is_none() {
        result.push(VecDeque::new());
        return result;
    }

    let mut prefix = VecDeque::new();
    node.map(|n| prefix.push_back(n.data.clone()));

    let mut left_sequence = all_sequences(node.and_then(|n| n.left.as_ref()));
    let mut right_sequence = all_sequences(node.and_then(|n| n.right.as_ref()));

    for left in left_sequence.iter_mut() {
        for right in right_sequence.iter_mut() {
            let mut weaved = Vec::new();
            weave_lists(left, right, &mut weaved, &mut prefix);
            result.append(&mut weaved);
        }
    }

    result
}

fn weave_lists<T>(first: &mut VecDeque<T>,
                  second: &mut VecDeque<T>,
                  results: &mut Vec<VecDeque<T>>,
                  prefix: &mut VecDeque<T>)
    where T: Clone
{
    if first.is_empty() || second.is_empty() {
        let mut result = prefix.clone();
        // Note: the append() method actually removes all the elements
        // from the second dynamic array (which is not what we want)!
        // The proper way to "append" the elements is to use extend
        // and pass in a cloned() iterator.
        result.extend(first.iter().cloned());
        result.extend(second.iter().cloned());
        results.push(result);
        return;
    }

    first.pop_front().map(|head_first| {
        prefix.push_back(head_first.clone());
        weave_lists(first, second, results, prefix);
        prefix.pop_back();
        first.push_front(head_first);
    });

    second.pop_front().map(|head_second| {
        prefix.push_back(head_second.clone());
        weave_lists(first, second, results, prefix);
        prefix.pop_back();
        second.push_front(head_second);
    });
}

#[cfg(test)]
mod tests {
    use super::super::Tree;
    use super::*;

    #[test]
    fn test_all_sequences_basic() {
        let tree = Tree {
            data: 2,
            left: Some(Box::new(Tree {
                data: 1,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Tree {
                data: 3,
                left: None,
                right: None,
            })),
        };

        let sequences = all_sequences(Some(&Box::new(tree)));
        assert_eq!(sequences,
                   vec![vec![2, 1, 3].into_iter().collect(), vec![2, 3, 1].into_iter().collect()]);
    }
}
