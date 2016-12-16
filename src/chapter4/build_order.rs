use std::collections::HashMap;
use super::Node;

pub unsafe fn find_projects(projects: Vec<String>,
                            dependencies: Vec<(String, String)>)
                            -> Vec<*mut Node<String>> {
    let mut graph = Graph::build(projects, dependencies);
    order_projects(&mut graph)
}

struct Graph {
    nodes: Vec<*mut Node<String>>,
    map: HashMap<String, *mut Node<String>>,
    dependencies: HashMap<*mut Node<String>, u32>,
}

impl Graph {
    pub fn build(projects: Vec<String>, dependencies: Vec<(String, String)>) -> Graph {
        let mut graph = Graph {
            nodes: Vec::new(),
            map: HashMap::new(),
            dependencies: HashMap::new(),
        };

        for project in projects {
            graph.add_project(project);
        }

        for (start, end) in dependencies {
            graph.add_edge(start, end);
        }

        graph
    }

    pub fn add_project(&mut self, data: String) {
        unsafe {
            let node = Node::new(data);
            self.nodes.push(node);
            self.map.insert((*node).data.clone(), node);
            self.dependencies.insert(node, 0);
        }
    }

    pub fn add_edge(&mut self, start: String, end: String) {
        if let (Some(n1), Some(n2)) = (self.map.get(&start).clone(), self.map.get(&end).clone()) {
            unsafe {
                // Check if the node already has the edge before adding it.
                if !(**n1).edges.contains(n2) {
                    (**n1).edges.push(*n2);
                    self.dependencies.get_mut(&n2).map(|d| *d += 1);
                }
            }
        }
    }
}

unsafe fn order_projects(graph: &mut Graph) -> Vec<*mut Node<String>> {
    let mut order = Vec::with_capacity(graph.nodes.len());

    // Add the root nodes with no dependencies.
    add_non_dependants(&mut order, &graph.nodes, &graph.dependencies);

    let mut order_index = 0;
    while order_index < order.len() {
        let project = order[order_index];

        // Decrement the children's dependencies and add
        // the children with no dependencies.
        for child in (*project).edges.iter() {
            graph.dependencies.get_mut(child).map(|d| *d -= 1);
        }
        add_non_dependants(&mut order, &(*project).edges, &graph.dependencies);

        order_index += 1;
    }

    order
}

unsafe fn add_non_dependants(order: &mut Vec<*mut Node<String>>,
                             projects: &Vec<*mut Node<String>>,
                             dependencies: &HashMap<*mut Node<String>, u32>) {
    for project in projects.iter() {
        let num_dependants = dependencies.get(project).cloned().unwrap_or(0);
        if num_dependants == 0 {
            order.push(*project);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use super::super::Node;
    use super::*;

    /// Helper function for testing the topological sort.
    fn test_expected(projects: Vec<String>,
                     edges: Vec<(String, String)>,
                     expected_output: Vec<String>) {
        unsafe {
            let nodes = find_projects(projects, edges);
            assert_eq!(nodes.len(), expected_output.len());

            for (node, expected) in nodes.into_iter().zip(expected_output.into_iter()) {
                assert_eq!((*node).data, expected);

                // Free the node.
                mem::transmute::<*mut Node<String>, Box<Node<String>>>(node);
            }
        }
    }

    #[test]
    fn test_basic_example() {
        let projects: Vec<_> =
            vec!["a", "b", "c", "d", "e", "f"].iter().map(|s| s.to_string()).collect();
        let edges: Vec<_> = vec![("a", "d"), ("f", "b"), ("b", "d"), ("f", "a"), ("d", "c")]
            .iter()
            .map(|&(a, b)| (a.to_string(), b.to_string()))
            .collect();
        let expected_output: Vec<_> =
            vec!["e", "f", "b", "a", "d", "c"].iter().map(|s| s.to_string()).collect();

        test_expected(projects, edges, expected_output);
    }

    #[test]
    fn test_repeated_edges() {
        let projects: Vec<_> = vec!["a", "b"].iter().map(|s| s.to_string()).collect();
        let edges: Vec<_> = vec![("a", "b"), ("a", "b"), ("a", "b")]
            .iter()
            .map(|&(a, b)| (a.to_string(), b.to_string()))
            .collect();
        let expected_output: Vec<_> = vec!["a", "b"].iter().map(|s| s.to_string()).collect();

        test_expected(projects, edges, expected_output);
    }

    #[test]
    fn test_cycle() {
        let projects: Vec<_> = vec!["a", "b", "c"].iter().map(|s| s.to_string()).collect();
        let edges: Vec<_> = vec![("a", "b"), ("b", "c"), ("c", "a")]
            .iter()
            .map(|&(a, b)| (a.to_string(), b.to_string()))
            .collect();

        test_expected(projects, edges, vec![]);
    }
}
