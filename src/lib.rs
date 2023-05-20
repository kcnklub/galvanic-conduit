pub mod connectivity_model;

#[derive(Debug)]
pub struct Node {
    id: String,
    node_type: NodeType,
    node_data: NodeData,
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
    id: String,
    source: String,
    downstream: String,
}

#[derive(Debug)]
pub struct SourceData {
    id: String,
    downstream: String,
}
