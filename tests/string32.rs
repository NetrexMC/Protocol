use binary_utils::*;
use mcpe_protocol::interfaces::String32;

#[test]
fn write_var_string() {
     let my_string = "Hello World!".to_string();
     let encoded = String32(my_string).parse();
     assert_eq!(&encoded[..], &[0, 0, 0, 12, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]);
}

#[test]
fn read_var_string() {
     let my_string = "Bob Ross".to_string();
     let known: &[u8] = &[0, 0, 0, 8, 66, 111, 98, 32, 82, 111, 115, 115];
     let data = String32::compose(&known, &mut 0);
     assert_eq!(my_string, data.0);
}