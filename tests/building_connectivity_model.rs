use claims::{assert_err, assert_ok, assert_some};
use connectivity_model::connectivity_model::{ConnectivityModel, Parsable};
use std::string::String;

#[derive(Debug)]
struct TestMetaData {
    non_unique_data: String,
}

/// This is a wrapper struct for a string so that I am impl Parsable.
impl Parsable for TestMetaData {
    fn parse(input: &Vec<&str>) -> Self {
        Self {
            non_unique_data: String::from(input[1]),
        }
    }
}

#[derive(Debug)]
struct TestConnectivityData {
    upstream: String,
}

impl Parsable for TestConnectivityData {
    fn parse(input: &Vec<&str>) -> Self {
        Self {
            upstream: String::from(input[2]),
        }
    }
}

type TestConnectivityModel = ConnectivityModel<TestMetaData, TestConnectivityData>;

#[test]
fn no_file_found() {
    let model = TestConnectivityModel::build_from_csv("file_does_not_exist.csv");

    assert_err!(model);
}

#[test]
fn file_found_but_empty() {
    let model = TestConnectivityModel::build_from_csv("test_resources/empty.csv");

    assert_ok!(model);
}

#[test]
fn build_single_node_model() {
    let result = TestConnectivityModel::build_from_csv("test_resources/single_node_model.csv");

    assert_ok!(&result);
    let model = result.unwrap();
    let node = model.get_node("node1");
    assert_some!(node);
    let node = node.unwrap();

    assert_eq!("test_data", node.meta_data.non_unique_data);
    assert_eq!("connectivity_data", node.connectivity_data.upstream)
}
