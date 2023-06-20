use crate::connectivity_model::Parsable;

#[derive(Debug)]
pub enum NodeType {
    Meter,
    Span,
    Source,
}

impl Parsable for NodeType {
    fn parse(_input: &[&str]) -> Self {
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
    fn parse(_input: &[&str]) -> Self {
        NodeData::Meter(MeterData {
            id: "something".to_string(),
            source: "something".to_string(),
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

#[cfg(test)]
mod test {}
