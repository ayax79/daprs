extern crate daprs_derive;
use daprs_derive::Stateful;
use daprs_core::state::Stateful;

#[derive(Stateful, Clone, Debug)]
struct TestStruct {        
}

#[test]
pub fn test_derive() {
    assert_eq!("TestStruct", TestStruct::key())
}
