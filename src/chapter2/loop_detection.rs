use std::fmt::Debug;
use std::{cell::Cell, ptr};
use typed_arena::Arena;

pub struct Node<'a, T> {
    pub data: T,
    pub next: Cell<Option<&'a Node<'a, T>>>,
}

impl<'a, T> Debug for &'a Node<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ data: {:?} }}", self.data)
    }
}

impl<'a, T> PartialEq for &'a Node<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(*self, *other)
    }
}

pub fn list_from_vec<'a, T>(v: Vec<T>, arena: &'a Arena<Node<'a, T>>) -> Option<&'a Node<'a, T>> {
    let mut prev = None;
    for v in v.into_iter().rev() {
        let node = arena.alloc(Node {
            data: v,
            next: Cell::new(prev),
        });
        prev = Some(node);
    }
    prev
}

pub fn vec_from_list<'a, T: Clone>(node: &'a Node<'a, T>) -> Vec<T> {
    let mut vec = Vec::new();
    let mut curr_node = Some(node);
    while let Some(node) = curr_node {
        vec.push(node.data.clone());
        curr_node = node.next.get();
    }
    vec
}

pub fn find_beginning<'a, T>(l: &'a Node<'a, T>) -> Option<&'a Node<'a, T>> {
    let mut faster = l.next.get().and_then(|n| n.next.get());
    let mut slower = l.next.get();

    // Get intersection of the faster and slower nodes.
    while let (Some(s), Some(f)) = (slower, faster) {
        if s == f {
            break;
        }

        slower = s.next.get();
        faster = f.next.get().and_then(|f| f.next.get());
    }

    // Return early if not loop.
    faster?;

    // Get intersection starting from the intersected node and the head of the list.
    let mut start = Some(l);
    let mut intersection = faster;
    while let (Some(s), Some(i)) = (start, intersection) {
        if s == i {
            return Some(s);
        }

        start = s.next.get();
        intersection = i.next.get();
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_to_vec() {
        let arena = Arena::new();
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let node = list_from_vec(v.clone(), &arena);
        assert_eq!(vec_from_list(node.unwrap()), v);

        let v: Vec<i32> = Vec::new();
        let node = list_from_vec(v.clone(), &arena);
        assert_eq!(node, None);
    }

    #[test]
    fn test_find_beginning() {
        let arena = Arena::new();
        let node = list_from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], &arena);
        assert_eq!(find_beginning(node.unwrap()), None);

        let mut eleventh = node;
        for _ in 0..10 {
            eleventh = eleventh.and_then(|n| n.next.get());
        }
        assert_eq!(eleventh.unwrap().data, 11);

        let mut fifth = node;
        for _ in 0..4 {
            fifth = fifth.and_then(|n| n.next.get());
        }
        assert_eq!(fifth.unwrap().data, 5);

        eleventh.unwrap().next.set(fifth);
        assert_eq!(find_beginning(node.unwrap()), fifth);
    }
}
