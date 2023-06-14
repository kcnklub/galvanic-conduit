use std::{collections::HashMap, fs, io::Error};

use crate::nodes::Node;

/// Basic Connectivity model that holds all nodes in a hashmap to be tracked.
#[derive(Debug)]
pub struct ConnectivityModel<NodeMetaData, NodeConnectivityData>
where
    NodeMetaData: Parsable,
    NodeConnectivityData: Parsable,
{
    nodes: HashMap<String, Node<NodeMetaData, NodeConnectivityData>>,
}

pub trait Parsable {
    fn parse(input: &Vec<&str>) -> Self;
}

impl<NodeMetaData, NodeConnectivityData> ConnectivityModel<NodeMetaData, NodeConnectivityData>
where
    NodeMetaData: Parsable,
    NodeConnectivityData: Parsable,
{
    pub fn get_node(&self, id: &str) -> Option<&Node<NodeMetaData, NodeConnectivityData>> {
        self.nodes.get(id)
    }

    pub fn add_node(&mut self, node: Node<NodeMetaData, NodeConnectivityData>) {
        let node_id = node.id.clone();
        self.nodes.insert(node_id, node);
    }

    pub fn build_from_csv(file_name: &str) -> Result<Self, Error> {
        let data = fs::read_to_string(file_name)?;
        let mut map: HashMap<String, Node<NodeMetaData, NodeConnectivityData>> = HashMap::new();
        for line in data.lines() {
            let split_line = line.split(", ").collect::<Vec<&str>>();

            let node_id = String::from(split_line[0]);
            let node = Node::<NodeMetaData, NodeConnectivityData> {
                id: node_id.clone(),
                meta_data: NodeMetaData::parse(&split_line),
                connectivity_data: NodeConnectivityData::parse(&split_line),
            };
            map.insert(node_id, node);
        }
        Ok(Self { nodes: map })
    }
}

#[cfg(test)]
mod test {
    use crate::nodes::{MeterData, Node, NodeData, NodeType};
    use claims::{assert_none, assert_some};
    use std::collections::HashMap;

    use super::{ConnectivityModel, Parsable};

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

    impl Parsable for String {
        fn parse(_input: &Vec<&str>) -> Self {
            String::from("something")
        }
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
