pub mod remove_duplicates;

use std::fmt::Debug;
use std::mem;

/// A singly-linked list node.
pub struct Node<T> {
    data: T,
    next: Option<*mut Node<T>>,
}

pub unsafe fn single_linked_list_from_vec<T>(v: Vec<T>) -> *mut Node<T> {
    let mut head: Option<*mut Node<T>> = None;
    let mut tail: Option<*mut Node<T>> = None;
    for elem in v {
        let new_node = Box::new(Node {
            data: elem,
            next: None,
        });
        let new_node_ptr = mem::transmute::<Box<Node<T>>, *mut Node<T>>(new_node);

        if let Some(tail_ptr) = tail.take() {
            (*tail_ptr).next = Some(new_node_ptr);
            tail = Some(new_node_ptr);
        } else {
            head = Some(new_node_ptr);
            tail = head;
        }
    }

    head.unwrap()
}

pub unsafe fn free_single_linked_list<T>(n: *mut Node<T>) {
    let mut curr = Some(n);
    while let Some(node) = curr.take() {
        curr = (*node).next;
        mem::transmute::<*mut Node<T>, Box<Node<T>>>(node);
    }
}

pub unsafe fn compare_single_linked_list<T>(n: *mut Node<T>, v: Vec<T>)
    where T: Debug + PartialEq
{
    let mut iter = v.into_iter();
    let mut curr = Some(n);
    while let Some(node) = curr.take() {
        assert_eq!((*node).data, iter.next().unwrap());
        curr = (*node).next;
    }
}
