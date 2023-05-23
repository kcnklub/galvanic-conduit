use std::collections::HashMap;

use crate::{Node, NodeType};

/// Basic Connectivity model that holds all nodes in a hashmap to be tracked.
pub struct ConnectivityModel {
    nodes: HashMap<String, Node>,
}

impl ConnectivityModel {
    fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// This is very slow because it looks over every node that in the the model at the moment.
    fn get_sources_nodes(&self) -> Vec<&Node> {
        let mut nodes_found: Vec<&Node> = vec![];
        for (_, value) in self.nodes.iter() {
            if let NodeType::Source = value.node_type {
                nodes_found.push(value);
            }
        }
        nodes_found
    }

    fn add_node(&mut self, node: Node) {
        let node_id = node._id.clone();
        self.nodes.insert(node_id, node);
    }
}

#[cfg(test)]
mod test {
    use crate::{MeterData, Node, NodeData};
    use claims::{assert_none, assert_some};
    use std::collections::HashMap;

    use super::{ConnectivityModel};

    #[test]
    fn get_node() {
        // arrange
        let mut map = HashMap::new();
        let meter_data = MeterData {
            id: "a".to_string(),
            source: "span_a".to_string(),
        };

        let node = Node {
            _id: "a".to_string(),
            node_type: crate::NodeType::Meter,
            _node_data: NodeData::Meter(meter_data),
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
        let connectivity_model = ConnectivityModel { nodes: map };

        // act
        let output = connectivity_model.get_node("a");

        // assert
        assert_none!(output);
    }

    #[test]
    fn get_source() {
        // arrange
        let map = HashMap::new();
        let connectivity_model = ConnectivityModel { nodes: map };

        // act
        let output = connectivity_model.get_sources_nodes();

        // assert
        assert!(output.is_empty());
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

        let node = Node {
            _id: "a".to_string(),
            node_type: crate::NodeType::Meter,
            _node_data: NodeData::Meter(meter_data),
        };

        // act
        connectivity_model.add_node(node);

        // assert
        let output = connectivity_model.get_node("a");
        assert_some!(output);
    }

}
