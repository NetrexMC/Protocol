use binary_utils::*;
use mcpe_protocol::interfaces::VarString;

#[test]
fn write_var_string() {
     let my_string = "Hello World!".to_string();
     let encoded = VarString(my_string).parse();
     assert_eq!(&encoded[..], &[12, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]);
}

#[test]
fn read_var_string() {
     let my_string = "Hello World!".to_string();
     let string = VarString::compose(&[12, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33], &mut 0);
     assert_eq!(string.0, my_string);
}