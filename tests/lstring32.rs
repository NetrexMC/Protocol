use binary_utils::*;
use mcpe_protocol::interfaces::LString32;

pub const HW_TEST_DATA: &[u8] = &[
     // Length of the string in Little Endian Format
     12, 0, 0, 0,
     // Contents of string
     72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33
];

#[test]
fn write_l32string() {
     let hello_world = "Hello World!".to_string();
     let data = LString32(hello_world).parse();

     assert_eq!(HW_TEST_DATA, &data[..]);
}

#[test]
fn read_l32string() {
     let hello_world = "Hello World!".to_string();
     let data = LString32::compose(HW_TEST_DATA, &mut 0);
     assert_eq!(data.0, hello_world);
}