pub mod remove_duplicates;
pub mod kth_to_last;
pub mod delete_middle_node;
pub mod partition;
pub mod sum_lists;
pub mod is_palindrome;
pub mod intersection;
pub mod loop_detection;

use std::fmt::Debug;

/// A singly-linked list node.
pub struct Node<T> {
    data: T,
    next: Option<*mut Node<T>>,
}

impl<T> Node<T> {
    pub unsafe fn new(data: T) -> *mut Node<T> {
        let new_node = Box::new(Node {
            data: data,
            next: None,
        });

        Box::into_raw(new_node)
    }
}

pub unsafe fn single_linked_list_len<T>(n: *mut Node<T>) -> i32 {
    let mut len = 0;
    let mut curr = Some(n);
    while let Some(node) = curr {
        len += 1;
        curr = (*node).next;
    }

    len
}

pub unsafe fn single_linked_list_from_vec<T>(v: Vec<T>) -> *mut Node<T> {
    let mut head: Option<*mut Node<T>> = None;
    let mut tail: Option<*mut Node<T>> = None;
    for elem in v {
        let new_node = Node::new(elem);

        if let Some(tail_ptr) = tail {
            (*tail_ptr).next = Some(new_node);
            tail = Some(new_node);
        } else {
            head = Some(new_node);
            tail = head;
        }
    }

    head.unwrap()
}

pub unsafe fn append_list<T>(l: *mut Node<T>, node: *mut Node<T>) {
    let mut curr = Some(l);

    while !curr.and_then(|n| (*n).next).is_none() {
        curr = curr.and_then(|n| (*n).next);
    }

    curr.map(move |n| (*n).next = Some(node));
}

pub unsafe fn free_single_linked_list<T>(n: *mut Node<T>) {
    let mut curr = Some(n);
    while let Some(node) = curr {
        curr = (*node).next;
        Box::from_raw(node);
    }
}

pub unsafe fn compare_single_linked_list<T>(n: *mut Node<T>, v: Vec<T>)
    where T: Debug + PartialEq
{
    let mut iter = v.into_iter();
    let mut curr = Some(n);
    while let Some(node) = curr {
        assert_eq!((*node).data, iter.next().unwrap());
        curr = (*node).next;
    }
}
