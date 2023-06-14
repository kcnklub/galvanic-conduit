use crate::connectivity_model::Parsable;

#[derive(Debug)]
pub enum NodeType {
    Meter,
    Span,
    Source,
}

impl Parsable for NodeType {
    fn parse(_input: &Vec<&str>) -> Self {
        NodeType::Meter
    }
}

#[derive(Debug)]
pub enum NodeData {
    Meter(MeterData),
    Span(SpanData),
    Source(SourceData),
}

impl Parsable for NodeData {
    fn parse(_input: &Vec<&str>) -> Self {
        NodeData::Meter(MeterData {
            id: "something".to_string(),
            source: "soemthing".to_string(),
        })
    }
}

#[derive(Debug)]
pub struct MeterData {
    pub id: String,
    pub source: String,
}

#[derive(Debug)]
pub struct SpanData {
    _id: String,
    _source: String,
    _downstream: String,
}

#[derive(Debug)]
pub struct SourceData {
    _id: String,
    _downstream: String,
}

#[derive(Debug)]
pub struct Node<MetaData, ConnectivityData> {
    pub id: String,
    pub meta_data: MetaData,
    pub connectivity_data: ConnectivityData,
}

#[cfg(test)]
mod test {
    use crate::nodes::Node;

    #[test]
    fn test() {
        let node = Node::<String, String> {
            id: "blah".to_string(),
            meta_data: "meta".to_string(),
            connectivity_data: "connectivity".to_string(),
        };

        assert_eq!("meta", node.meta_data);
        assert_eq!("connectivity", node.connectivity_data);
    }
}
