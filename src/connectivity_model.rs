use std::collections::HashMap;

use crate::{Node, NodeType};

trait GetNodes {
    fn get_node(&self, id: &String) -> Option<&Node>;

    fn get_sources_nodes(&self) -> Vec<&Node>;
}

/// Basic Connectivity model that holds all nodes in a hashmap to be tracked.
pub struct ConnectivityModel {
    nodes: HashMap<String, Node>,
}

impl GetNodes for ConnectivityModel {
    fn get_node(&self, id: &String) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// This is very slow because it looks over every node that in the the model at the moment.
    fn get_sources_nodes(&self) -> Vec<&Node> {
        let mut nodes_found: Vec<&Node> = vec![];
        for (_, value) in self.nodes.iter() {
            match value.node_type {
                NodeType::Source => nodes_found.push(&value),
                _ => {}
            }
        }
        nodes_found
    }
}

#[cfg(test)]
mod test {
    use crate::{MeterData, Node, NodeData};
    use claims::{assert_none, assert_some};
    use std::collections::HashMap;

    use super::{ConnectivityModel, GetNodes};

    #[test]
    fn get_node() {
        // arrange
        let mut map = HashMap::new();
        let meter_data = MeterData {
            id: "a".to_string(),
            source: "span_a".to_string(),
        };

        let node = Node {
            id: "a".to_string(),
            node_type: crate::NodeType::Meter,
            node_data: NodeData::Meter(meter_data),
        };

        map.insert("a".to_string(), node);
        let connectivity_model = ConnectivityModel { nodes: map };

        // act
        let output = connectivity_model.get_node(&"a".to_string());

        // assert
        assert_some!(output);
    }

    #[test]
    fn no_node_in_map() {
        // arrange
        let map = HashMap::new();
        let connectivity_model = ConnectivityModel { nodes: map };

        // act
        let output = connectivity_model.get_node(&"a".to_string());

        // assert
        assert_none!(output);
    }

    #[test]
    fn get_source() {
        let map = HashMap::new();
        let connectivity_model = ConnectivityModel { nodes: map };

        // act
        let output = connectivity_model.get_sources_nodes();

        // assert
        assert!(output.is_empty());
    }
}
