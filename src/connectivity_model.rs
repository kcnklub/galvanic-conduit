use std::{collections::HashMap, ptr::addr_of};

use crate::Node;

trait GetNodes {
    fn get_nodes(&self, id: &String) -> Option<&Node>;
}

struct ConnectivityModel {
    nodes: HashMap<String, Node>,
}

impl GetNodes for ConnectivityModel {
    fn get_nodes(&self, id: &String) -> Option<&Node> {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::{MeterData, Node, SpanData};
    use std::collections::HashMap;

    use super::{ConnectivityModel, GetNodes};

    #[test]
    fn get_node() {
        let mut map = HashMap::new();

        let meter_data = MeterData {
            id: "a".to_string(),
            source: "span_a".to_string(),
        };
        map.insert(meter_data.id.clone(), Node::Meter(meter_data));

        let connectivity_model = ConnectivityModel { nodes: map };

        let output = connectivity_model.get_nodes(&"a".to_string());
    }
}
