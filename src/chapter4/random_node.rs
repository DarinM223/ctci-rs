extern crate rand;

use rand::distributions::{IndependentSample, Range};

/// Basic tree interface that is required to implement.
pub trait Tree<T>: Sized {
    fn data(&self) -> T;
    fn len(&self) -> i32;
    fn is_empty(&self) -> bool;
    fn random_node(&self) -> Option<Self>;
    fn insert_in_order(&mut self, d: T);
    fn find(&self, d: T) -> Option<Self>;
}

#[derive(Clone)]
pub struct RandTree<T: Clone> {
    node_data: T,
    size: i32,
    left: Option<Box<RandTree<T>>>,
    right: Option<Box<RandTree<T>>>,
}

impl<T> RandTree<T>
where
    T: Clone + PartialOrd,
{
    pub fn new(data: T) -> RandTree<T> {
        RandTree {
            node_data: data,
            left: None,
            right: None,
            size: 1,
        }
    }

    /// Helper function used by OptRandTree in order to get the ith
    /// in order node in the tree.
    pub fn get_ith(&self, i: i32) -> Option<RandTree<T>> {
        let left_size = self.left.as_deref().map(|n| n.size).unwrap_or(0);
        match i.cmp(&left_size) {
            std::cmp::Ordering::Less => self.left.as_deref().and_then(|n| n.get_ith(i)),
            std::cmp::Ordering::Greater => {
                // Subtract the left nodes from i so that when get_ith is called again
                // on the right node it can correctly check the left node for the size.
                self.right
                    .as_deref()
                    .and_then(|n| n.get_ith(i - (left_size + 1)))
            }
            std::cmp::Ordering::Equal => Some(self.clone()),
        }
    }
}

impl<T> Tree<T> for RandTree<T>
where
    T: Clone + PartialOrd,
{
    fn data(&self) -> T {
        self.node_data.clone()
    }

    fn len(&self) -> i32 {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Recursive function that returns a truely random node
    /// in the tree (each node has the same probability of coming up).
    ///
    /// It does this by giving the probability of the random
    /// node being on the left to be left_size / total_size
    /// and the probability of the random node being on the right
    /// to be right_size / total_size and the probability of the random
    /// node being the head to be 1 / total_size.
    fn random_node(&self) -> Option<RandTree<T>> {
        let left_size = self.left.as_deref().map(|n| n.size).unwrap_or(0);

        let between = Range::new(0, self.size);
        let mut rng = rand::thread_rng();
        let index = between.ind_sample(&mut rng);

        match index.cmp(&left_size) {
            std::cmp::Ordering::Less => self.left.as_deref().and_then(|n| n.random_node()),
            std::cmp::Ordering::Greater => self.right.as_deref().and_then(|n| n.random_node()),
            std::cmp::Ordering::Equal => Some(self.clone()),
        }
    }

    /// Recursive function that inserts the data into either the left or right children
    /// and updates the node's size.
    fn insert_in_order(&mut self, d: T) {
        if d <= self.node_data {
            if let Some(ref mut left) = self.left {
                left.insert_in_order(d);
            } else {
                self.left = Some(Box::new(RandTree::new(d)));
            }
        } else if let Some(ref mut right) = self.right {
            right.insert_in_order(d);
        } else {
            self.right = Some(Box::new(RandTree::new(d)));
        }

        self.size += 1;
    }

    fn find(&self, d: T) -> Option<RandTree<T>> {
        if d == self.node_data {
            Some(self.clone())
        } else if d <= self.node_data {
            self.left.as_deref().and_then(|n| n.find(d))
        } else if d > self.node_data {
            self.right.as_deref().and_then(|n| n.find(d))
        } else {
            None
        }
    }
}

/// An optimized tree that only generates the random number once.
pub struct OptRandTree<T: Clone> {
    tree: RandTree<T>,
}

impl<T> OptRandTree<T>
where
    T: Clone + PartialOrd,
{
    pub fn new(data: T) -> OptRandTree<T> {
        OptRandTree {
            tree: RandTree::new(data),
        }
    }
}

impl<T> Tree<T> for OptRandTree<T>
where
    T: Clone + PartialOrd,
{
    fn data(&self) -> T {
        self.tree.data()
    }

    fn len(&self) -> i32 {
        self.tree.len()
    }

    fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    /// Generates a random number from 0 to i and
    /// then calls the tree's get_ith function.
    fn random_node(&self) -> Option<Self> {
        let between = Range::new(0, self.len());
        let mut rng = rand::thread_rng();
        let index = between.ind_sample(&mut rng);

        self.tree.get_ith(index).map(|n| OptRandTree { tree: n })
    }

    fn insert_in_order(&mut self, d: T) {
        self.tree.insert_in_order(d);
    }

    fn find(&self, d: T) -> Option<Self> {
        self.tree.find(d).map(|n| OptRandTree { tree: n })
    }
}

#[cfg(test)]
mod tests {
    extern crate timebomb;

    use self::timebomb::timeout_ms;
    use super::*;
    use std::collections::HashSet;
    use std::hash::Hash;

    fn make_hash_set(arr: Vec<i32>) -> HashSet<i32> {
        let mut set = HashSet::with_capacity(arr.len());
        for v in arr {
            set.insert(v);
        }

        set
    }

    fn populate_tree<V, T: Tree<V>>(elems: &HashSet<V>, tree: &mut T)
    where
        V: Clone + PartialOrd + Hash + Eq,
    {
        for v in elems.iter() {
            tree.insert_in_order(v.clone());
        }
    }

    fn check_random_coverage<V, T: Tree<V>>(elems: &mut HashSet<V>, tree: &T)
    where
        V: Clone + PartialOrd + Hash + Eq,
    {
        while !elems.is_empty() {
            let num = tree.random_node().as_ref().map(|n| n.data());

            match num {
                Some(ref num) if elems.contains(num) => {
                    elems.remove(num);
                }
                _ => {}
            }
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_random_node() {
        let mut set = make_hash_set(vec![3, 1, 4, 8, 6, 9]);
        let mut tree = RandTree::new(5);

        populate_tree(&set, &mut tree);

        assert_eq!(tree.len(), 7);
        assert_eq!(tree.data(), 5);
        assert!(tree.find(9).is_some());

        timeout_ms(
            move || {
                check_random_coverage(&mut set, &tree);
            },
            3000,
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_optimized_random_node() {
        let mut set = make_hash_set(vec![3, 1, 4, 8, 6, 9]);
        let mut tree = OptRandTree::new(5);

        populate_tree(&set, &mut tree);

        assert_eq!(tree.len(), 7);
        assert_eq!(tree.data(), 5);
        assert!(tree.find(9).is_some());

        timeout_ms(
            move || {
                check_random_coverage(&mut set, &tree);
            },
            3000,
        );
    }
}
