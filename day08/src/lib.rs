use std::collections::HashMap;
use std::slice::Iter;

type NodeID = u32;

struct Node {
    num_child: usize,
    num_meta: usize,
    child: Vec<NodeID>,
    metadata: Vec<u8>,
}

pub struct NodeGraph {
    nodes: HashMap<NodeID, Node>,
    next_node_id: NodeID,
}

impl NodeGraph {
    fn new() -> Self {
        NodeGraph {
            nodes: HashMap::new(),
            next_node_id: 0,
        }
    }

    pub fn build_graph(input: &[u8]) -> Self {
        let mut input_iter = input.iter();
        let mut stack = Vec::new();
        let mut graph = Self::new();

        loop {
            let num_child = *input_iter.next().expect("Invalid input") as usize;
            let num_meta = *input_iter.next().expect("Invalid input") as usize;
            let mut node_id = graph.add_node(num_child, num_meta);
            if num_child > 0 {
                stack.push(node_id);
                continue;
            }
            loop {
                graph.add_metadata(node_id, &mut input_iter);
                if let Some(parent_id) = stack.pop() {
                    graph.add_child(parent_id, node_id);
                    if graph.missing_child(parent_id) {
                        stack.push(parent_id);
                        break;
                    }
                    node_id = parent_id;
                } else {
                    return graph;
                }
            }
        }
    }

    pub fn sum_metadata(&self) -> u32 {
        self.nodes.values().map(|node| node.sum_metadata()).sum()
    }

    pub fn total_value(&self) -> u32 {
        let mut stack = Vec::new();
        let mut total = 0;

        stack.push(0);
        while let Some(node_id) = stack.pop() {
            let node = self.nodes.get(&node_id).expect("Invalid node ID");
            if !node.has_child() {
                total += node.sum_metadata();
            } else {
                // TODO: avoid visiting the same node multiple times
                stack.append(&mut node.get_child_by_metadata());
            }
        }
        total
    }

    fn add_node(&mut self, num_child: usize, num_meta: usize) -> NodeID {
        let node_id = self.next_node_id;
        self.nodes.insert(node_id, Node::new(num_child, num_meta));
        self.next_node_id += 1;
        node_id
    }

    fn add_child(&mut self, parent_id: NodeID, node_id: NodeID) {
        self.nodes
            .entry(parent_id)
            .and_modify(|node| node.add_child(node_id));
    }

    fn missing_child(&self, node_id: NodeID) -> bool {
        self.nodes
            .get(&node_id)
            .map_or(false, |node| node.missing_child())
    }

    fn add_metadata(&mut self, node_id: NodeID, metadata: &mut Iter<u8>) {
        self.nodes
            .entry(node_id)
            .and_modify(|node| node.add_metadata(metadata));
    }
}

impl Node {
    pub fn new(num_child: usize, num_meta: usize) -> Self {
        Node {
            num_child,
            num_meta,
            child: Vec::with_capacity(num_child),
            metadata: Vec::with_capacity(num_meta),
        }
    }

    pub fn add_child(&mut self, child_id: NodeID) {
        self.child.push(child_id);
        assert!(self.child.len() <= self.num_child);
    }

    pub fn missing_child(&self) -> bool {
        self.child.len() < self.num_child
    }

    pub fn add_metadata(&mut self, metadata: &mut Iter<u8>) {
        self.metadata.extend(metadata.take(self.num_meta));
        assert!(self.metadata.len() == self.num_meta);
    }

    pub fn sum_metadata(&self) -> u32 {
        self.metadata.iter().map(|val| u32::from(*val)).sum()
    }

    pub fn has_child(&self) -> bool {
        self.num_child > 0
    }

    pub fn get_child_by_metadata(&self) -> Vec<NodeID> {
        self.metadata
            .iter()
            .filter(|&val| *val > 0)
            .filter_map(|val| self.child.get(usize::from(val - 1)))
            .cloned()
            .collect()
    }
}

pub fn part1(graph: &NodeGraph) -> u32 {
    graph.sum_metadata()
}

pub fn part2(graph: &NodeGraph) -> u32 {
    graph.total_value()
}
