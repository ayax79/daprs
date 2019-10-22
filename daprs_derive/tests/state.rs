extern crate daprs_derive;
use daprs_core::state::Stateful;
use daprs_derive::Stateful;

#[derive(Stateful, Clone, Debug)]
struct TestStruct {}

#[test]
pub fn test_derive() {
    assert_eq!("TestStruct", TestStruct::key())
}
