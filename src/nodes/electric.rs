//! # Electrical Node types
//!
//! - This is supposed cover the basic node types for an electrical grid
use crate::connectivity_model::Parsable;

/// Represents a meta data for a meter in an electrical grid
pub struct MeterData {
    pub meter_number: String,
}

impl Parsable for MeterData {
    fn parse(input: &[&str]) -> Self {
        MeterData {
            meter_number: String::from(input[2]),
        }
    }
}

/// Represents the connectivity information for a meter in an electrical grid.
pub struct MeterConnectivityData {
    /// This is the upstream span from this meter.
    pub upstream: String,
}

impl Parsable for MeterConnectivityData {
    fn parse(input: &[&str]) -> Self {
        MeterConnectivityData {
            upstream: String::from(input[3]),
        }
    }
}

/// This represents metadata for a span (section of line).
pub struct SpanData {}

impl Parsable for SpanData {
    fn parse(_input: &[&str]) -> Self {
        SpanData {}
    }
}

pub struct SpanConnectivityData {
    pub upstream: String,
    pub downstream: String,
}

pub struct SubStationData {
    pub name: String,
}

impl Parsable for SubStationData {
    fn parse(input: &[&str]) -> Self {
        SubStationData {
            name: String::from(input[2]),
        }
    }
}

pub struct SubStationConnectivityData {
    pub downstream: String,
}

pub enum ElectricalMeterData {
    Meter(MeterData),
    Span(SpanData),
    SubStation(SubStationData),
}

impl Parsable for ElectricalMeterData {
    fn parse(input: &[&str]) -> Self {
        match input[1] {
            "meter" => Self::Meter(MeterData::parse(input)),
            "span" => Self::Span(SpanData::parse(input)),
            "substation" => Self::SubStation(SubStationData::parse(input)),
            _ => panic!("this is an unsupported type"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::connectivity_model::Parsable;

    use super::ElectricalMeterData;

    #[test]
    fn creating_meter_meta_data() {
        let input_data = ["id", "meter", "meter number", "my upstream"];

        let output = ElectricalMeterData::parse(&input_data);

        assert!(matches!(output, ElectricalMeterData::Meter(..)));
        match output {
            ElectricalMeterData::Meter(data) => {
                assert_eq!("meter number", data.meter_number)
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn creating_span_meta_data() {
        let input_data = ["id", "span", "meter number", "my upstream"];

        let output = ElectricalMeterData::parse(&input_data);

        assert!(matches!(output, ElectricalMeterData::Span(..)));
        match output {
            ElectricalMeterData::Span(_data) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn creating_substation_meta_data() {
        let input_data = ["id", "substation", "substation name", "down stream"];

        let output = ElectricalMeterData::parse(&input_data);

        assert!(matches!(output, ElectricalMeterData::SubStation(..)));

        match output {
            ElectricalMeterData::SubStation(data) => {
                assert_eq!("substation name", data.name)
            }
            _ => assert!(false),
        }
    }
}
