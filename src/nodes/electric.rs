use crate::connectivity_model::Parsable;

/// This is supposed cover the basic node types for an electrical grid

/// Represents a meta data for a meter in an electrical grid
struct MeterData {
    meter_number: String,
}

/// Represents the connectivity information for a meter in an electrical grid.
struct MeterConnectivityData {
    /// This is the upstream span from this meter.
    upstream: String,
}

/// This represent
struct SpanData {}

struct SpanConnectivityData {
    upstream: String,
    downstream: String,
}

struct SubStationData {
    name: String,
}

struct SubStationConnectivityData {
    downstream: String,
}
