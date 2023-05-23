use std::collections::HashMap;

use crate::nodes::Node;

/// Basic Connectivity model that holds all nodes in a hashmap to be tracked.
pub struct ConnectivityModel<NodeMetaData, NodeConnectivityData> {
    nodes: HashMap<String, Node<NodeMetaData, NodeConnectivityData>>,
}

impl<NodeMetaData, NodeConnectivityData> ConnectivityModel<NodeMetaData, NodeConnectivityData> {
    pub fn get_node(&self, id: &str) -> Option<&Node<NodeMetaData, NodeConnectivityData>> {
        self.nodes.get(id)
    }

    pub fn add_node(&mut self, node: Node<NodeMetaData, NodeConnectivityData>) {
        let node_id = node.id.clone();
        self.nodes.insert(node_id, node);
    }
}

#[cfg(test)]
mod test {
    use crate::nodes::{MeterData, Node, NodeData, NodeType};
    use claims::{assert_none, assert_some};
    use std::collections::HashMap;

    use super::ConnectivityModel;

    #[test]
    fn get_node() {
        // arrange
        let mut map = HashMap::new();
        let meter_data = MeterData {
            id: "a".to_string(),
            source: "span_a".to_string(),
        };

        let node = Node::<NodeType, NodeData> {
            id: "a".to_string(),
            meta_data: NodeType::Meter,
            connectivity_data: NodeData::Meter(meter_data),
        };

        map.insert("a".to_string(), node);
        let connectivity_model = ConnectivityModel { nodes: map };

        // act
        let output = connectivity_model.get_node("a");

        // assert
        assert_some!(output);
    }

    #[test]
    fn no_node_in_map() {
        // arrange
        let map = HashMap::new();
        let connectivity_model = ConnectivityModel::<String, String> { nodes: map };

        // act
        let output = connectivity_model.get_node("a");

        // assert
        assert_none!(output);
    }

    #[test]
    fn add_node() {
        // arrange
        let map = HashMap::new();
        let mut connectivity_model = ConnectivityModel { nodes: map };
        let meter_data = MeterData {
            id: "a".to_string(),
            source: "span_a".to_string(),
        };

        let node = Node::<NodeType, NodeData> {
            id: "a".to_string(),
            meta_data: NodeType::Meter,
            connectivity_data: NodeData::Meter(meter_data),
        };

        // act
        connectivity_model.add_node(node);

        // assert
        let output = connectivity_model.get_node("a");
        assert_some!(output);
    }
}
