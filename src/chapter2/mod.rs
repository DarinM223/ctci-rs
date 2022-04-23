pub mod delete_middle_node;
pub mod intersection;
pub mod is_palindrome;
pub mod kth_to_last;
pub mod loop_detection;
pub mod partition;
pub mod remove_duplicates;
pub mod sum_lists;

#[derive(Clone, Debug, PartialEq)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub fn list_from_vec<T>(v: &mut Vec<T>) -> Node<T> {
    let mut node = Node {
        data: v.pop().unwrap(),
        next: None,
    };
    while let Some(data) = v.pop() {
        node = Node {
            data,
            next: Some(Box::new(node)),
        };
    }

    node
}

pub fn vec_from_list<T: Clone>(n: &Node<T>) -> Vec<T> {
    let mut vec = Vec::new();
    fn vec_from_list_iter<T: Clone>(n: &Node<T>, vec: &mut Vec<T>) {
        vec.push(n.data.clone());
        if let Some(next) = n.next.as_deref() {
            vec_from_list_iter(next, vec);
        }
    }

    vec_from_list_iter(n, &mut vec);
    vec
}
