use std::{collections::HashMap, fs, io::Error};

/// Basic Connectivity model that holds all nodes in a hashmap to be tracked.
#[derive(Debug)]
pub struct ConnectivityModel<NodeMetaData, NodeConnectivityData>
where
    NodeMetaData: Parsable,
    NodeConnectivityData: Parsable,
{
    nodes: HashMap<String, Node<NodeMetaData, NodeConnectivityData>>,
}

/// This is a trait that allows us to parse the data from the csv file into the
/// correct type.
pub trait Parsable {
    fn parse(input: &[&str]) -> Self;
}

impl<NodeMetaData, NodeConnectivityData> ConnectivityModel<NodeMetaData, NodeConnectivityData>
where
    NodeMetaData: Parsable,
    NodeConnectivityData: Parsable,
{
    pub fn get_node(&self, id: &str) -> Option<&Node<NodeMetaData, NodeConnectivityData>> {
        self.nodes.get(id)
    }

    pub fn build_from_csv(file_name: &str) -> Result<Self, Error> {
        let data = fs::read_to_string(file_name)?;
        let mut map: HashMap<String, Node<NodeMetaData, NodeConnectivityData>> = HashMap::new();
        for line in data.lines() {
            let split_line = line.split(',').collect::<Vec<&str>>();

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

#[derive(Debug)]
pub struct Node<NodeMetaData, NodeConnectivityData>
where
    NodeMetaData: Parsable,
    NodeConnectivityData: Parsable,
{
    pub id: String,
    pub meta_data: NodeMetaData,
    pub connectivity_data: NodeConnectivityData,
}
