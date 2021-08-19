use binary_utils::BinaryStream;

pub struct Login {
     pub protocol: u8,
     pub connection_request: BinaryStream
}