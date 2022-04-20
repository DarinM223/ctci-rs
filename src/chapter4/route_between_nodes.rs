use ghost_cell::{GhostCell, GhostToken};
use std::{
    collections::{HashSet, VecDeque},
    ptr,
};

pub struct Graph<'arena, 'id, T> {
    pub data: T,
    pub edges: Vec<&'arena GhostCell<'id, Graph<'arena, 'id, T>>>,
}

pub fn route_between_nodes<'arena, 'id, T>(
    node1: &'arena GhostCell<'id, Graph<'arena, 'id, T>>,
    node2: &'arena GhostCell<'id, Graph<'arena, 'id, T>>,
    token: &GhostToken<'id>,
) -> bool {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(node1);
    while let Some(node) = queue.pop_front() {
        if !visited.contains(&(node as *const _)) {
            if ptr::eq(node, node2) {
                return true;
            }
            for &edge in node.borrow(token).edges.iter() {
                queue.push_back(edge);
            }
            visited.insert(node as *const _);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use typed_arena::Arena;

    #[test]
    fn test_route_between_nodes() {
        GhostToken::new(|mut token| {
            let arena = Arena::new();
            let node1 = GhostCell::from_mut(arena.alloc(Graph {
                data: 1,
                edges: Vec::new(),
            }));
            let node2 = GhostCell::from_mut(arena.alloc(Graph {
                data: 2,
                edges: Vec::new(),
            }));
            let node3 = GhostCell::from_mut(arena.alloc(Graph {
                data: 3,
                edges: Vec::new(),
            }));
            node1.borrow_mut(&mut token).edges.push(node3);
            node2.borrow_mut(&mut token).edges.push(node3);
            assert_eq!(route_between_nodes(node1, node3, &token), true);
            assert_eq!(route_between_nodes(node2, node3, &token), true);
            assert_eq!(route_between_nodes(node1, node2, &token), false);
        });
    }
}
