use binary_utils::*;
use mcpe_protocol::interfaces::VarSlice;

#[test]
fn read_var_slice() {
     let slice_data: &[u8] = &[5, 255, 12, 10, 12, 21, 32, 32];
     let sliced = VarSlice::fcompose(&slice_data, &mut 0);

     assert_eq!(&sliced.0[..], &[255, 12, 10, 12, 21]);
}

#[test]
fn write_var_slice() {
     let to_write: VarSlice = VarSlice(vec![255, 12, 10, 12, 21]);
     assert_eq!(&to_write.fparse()[..], &[5, 255, 12, 10, 12, 21]);
}