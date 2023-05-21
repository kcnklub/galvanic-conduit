pub mod connectivity_model;

#[derive(Debug)]
pub struct Node {
    _id: String,
    node_type: NodeType,
    _node_data: NodeData,
}

#[derive(Debug)]
pub enum NodeType {
    Meter,
    Span,
    Source,
}

#[derive(Debug)]
pub enum NodeData {
    Meter(MeterData),
    Span(SpanData),
    Source(SourceData),
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
