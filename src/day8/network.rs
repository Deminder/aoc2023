use std::{fmt::Display, ops::Index};

use either::Either;

const NODE_COUNT: usize = 26 * 26 * 26;
const EMPTY_NODE: Node = Node(u16::MAX);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Node(u16);

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Self(
            value
                .chars()
                .fold(0_u16, |identity, c| identity * 26 + (c as u16 - 'A' as u16)),
        )
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mid = self.0 / 26;
        let first = mid / 26;
        let chars = [first, mid, self.0].map(|n| (b'A' + (n % 26) as u8) as char);
        write!(f, "{}", chars.into_iter().collect::<String>())
    }
}

impl Node {
    pub fn index(&self) -> usize {
        self.0 as usize
    }

    pub fn is_end_node(&self, part2: bool) -> bool {
        if part2 {
            self.0 % 26 == (b'Z' - b'A') as u16
        } else {
            self.0 == Into::<Self>::into("ZZZ").0
        }
    }

    pub fn start_nodes(part2: bool) -> impl Iterator<Item = Self> {
        if part2 {
            Either::Left((0..NODE_COUNT / 26).map(|i| Self(i as u16 * 26)))
        } else {
            Either::Right(["AAA".into()].into_iter())
        }
    }
}

/// Node collection which maps `start_node -> (left_node, right_node)`
/// (This is a fixed sized array which can be initialized "faster" than a HashMap)
pub struct NodeNetwork([(Node, Node); NODE_COUNT]);

impl NodeNetwork {
    pub fn start_nodes(&self, part2: bool) -> impl Iterator<Item = Node> + '_ {
        Node::start_nodes(part2).filter(|n| self.0[n.index()].0 != EMPTY_NODE)
    }
}

impl Index<Node> for NodeNetwork {
    type Output = (Node, Node);

    fn index(&self, index: Node) -> &Self::Output {
        &self.0[index.index()]
    }
}

impl FromIterator<[Node; 3]> for NodeNetwork {
    fn from_iter<T: IntoIterator<Item = [Node; 3]>>(iter: T) -> Self {
        let mut network = [(EMPTY_NODE, EMPTY_NODE); NODE_COUNT];
        for [start, left, right] in iter.into_iter() {
            network[start.index()] = (left, right);
        }
        Self(network)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_node() {
        for start_node in Node::start_nodes(false) {
            assert!(start_node.to_string().ends_with('A'));
        }

        let end_node: Node = "ZZZ".into();
        assert!(end_node.is_end_node(false));
        assert!(end_node.is_end_node(true));
        let end_node2: Node = "BBZ".into();
        assert!(!end_node2.is_end_node(false));
        assert!(end_node2.is_end_node(true));

        for no_end_node in ["ZBB", "BZB", "ZBB"] {
            let n: Node = no_end_node.into();
            assert!(!n.is_end_node(false));
            assert!(!n.is_end_node(true));
        }
    }

    #[test]
    fn test_find_next_end() {
        let nodes: [Node; 3] = ["AAA", "BBB", "CCC"].map(|v| v.into());
        let network: NodeNetwork = [nodes].into_iter().collect();
        assert_eq!(network[nodes[0]], (nodes[1], nodes[2]));
    }
}
