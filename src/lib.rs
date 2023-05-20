mod connectivity_model;

enum Node {
    Meter(MeterData),
    Span(SpanData),
    Source(SourceData),
}

struct MeterData {
    id: String,
    source: String,
}

struct SpanData {
    id: String,
    source: String,
    downstream: String,
}

struct SourceData {
    id: String,
    downstream: String,
}

#[cfg(test)]
mod test {
    use crate::{MeterData, Node, SpanData};

    #[test]
    fn test() {
        let meter_data = MeterData {
            id: "a".to_string(),
            source: "span_a".to_string(),
        };
        let meter = Node::Meter(meter_data);

        let span_data = SpanData {
            id: "span_a".to_string(),
            source: "source_a".to_string(),
            downstream: "a".to_string(),
        };
        let span = Node::Span(span_data);
    }
}
