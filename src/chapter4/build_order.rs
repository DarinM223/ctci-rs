use slotmap::{SlotMap, new_key_type};
use std::collections::HashMap;

new_key_type! { pub struct GraphKey; }
/// A graph node for an adjacency list graph structure.
pub struct GraphNode<T> {
    pub data: T,
    pub edges: Vec<GraphKey>,
}

pub type GraphNodes<T> = SlotMap<GraphKey, GraphNode<T>>;

pub fn find_projects(
    projects: Vec<String>,
    dependencies: Vec<(String, String)>,
    nodes: &mut GraphNodes<String>,
) -> Vec<GraphKey> {
    let mut graph = Graph::build(projects, dependencies, nodes);
    order_projects(&mut graph, nodes)
}

struct Graph {
    keys: Vec<GraphKey>,
    map: HashMap<String, GraphKey>,
    dependencies: HashMap<GraphKey, u32>,
}

impl Graph {
    pub fn build(
        projects: Vec<String>,
        dependencies: Vec<(String, String)>,
        nodes: &mut GraphNodes<String>,
    ) -> Graph {
        let mut graph = Graph {
            keys: Vec::with_capacity(projects.len()),
            map: HashMap::with_capacity(projects.len()),
            dependencies: HashMap::with_capacity(projects.len()),
        };

        for project in projects {
            graph.add_project(project, nodes);
        }

        for (start, end) in dependencies {
            graph.add_edge(start, end, nodes);
        }

        graph
    }

    pub fn add_project(&mut self, data: String, nodes: &mut GraphNodes<String>) {
        let node = nodes.insert(GraphNode {
            data: data.clone(),
            edges: Vec::new(),
        });
        self.keys.push(node);
        self.map.insert(data, node);
        self.dependencies.insert(node, 0);
    }

    pub fn add_edge(&mut self, start: String, end: String, nodes: &mut GraphNodes<String>) {
        if let (Some(&n1), Some(&n2)) = (self.map.get(&start), self.map.get(&end)) {
            if !nodes[n1].edges.contains(&n2) {
                nodes[n1].edges.push(n2);
                if let Some(d) = self.dependencies.get_mut(&n2) {
                    *d += 1;
                }
            }
        }
    }
}

fn order_projects(graph: &mut Graph, nodes: &mut GraphNodes<String>) -> Vec<GraphKey> {
    let mut order = Vec::with_capacity(graph.keys.len());

    // Add the root nodes with no dependencies.
    add_non_dependants(&mut order, &graph.keys, &graph.dependencies);

    let mut order_index = 0;
    while order_index < order.len() {
        let project = order[order_index];

        // Decrement the children's dependencies and add
        // the children with no dependencies.
        for child in nodes[project].edges.iter() {
            if let Some(d) = graph.dependencies.get_mut(child) {
                *d -= 1;
            }
        }
        add_non_dependants(&mut order, &nodes[project].edges, &graph.dependencies);

        order_index += 1;
    }

    order
}

fn add_non_dependants(
    order: &mut Vec<GraphKey>,
    projects: &[GraphKey],
    dependencies: &HashMap<GraphKey, u32>,
) {
    for project in projects.iter() {
        let num_dependants = dependencies.get(project).cloned().unwrap_or(0);
        if num_dependants == 0 {
            order.push(*project);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slotmap::SlotMap;

    /// Helper function for testing the topological sort.
    fn test_expected(
        projects: Vec<String>,
        edges: Vec<(String, String)>,
        expected_output: Vec<String>,
    ) {
        let mut nodes = SlotMap::with_key();
        let keys = find_projects(projects, edges, &mut nodes);
        assert_eq!(keys.len(), expected_output.len());

        for (node, expected) in keys.into_iter().zip(expected_output.into_iter()) {
            assert_eq!(nodes[node].data, expected);
        }
    }

    #[test]
    fn test_basic_example() {
        let projects: Vec<_> = vec!["a", "b", "c", "d", "e", "f"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let edges: Vec<_> = vec![("a", "d"), ("f", "b"), ("b", "d"), ("f", "a"), ("d", "c")]
            .iter()
            .map(|&(a, b)| (a.to_string(), b.to_string()))
            .collect();
        let expected_output: Vec<_> = vec!["e", "f", "b", "a", "d", "c"]
            .iter()
            .map(|s| s.to_string())
            .collect();

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
