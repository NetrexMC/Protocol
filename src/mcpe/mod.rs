use binary_utils::*;
use crate::interfaces::{VarString, Slice};
// This file contains all packet encoding for Netrex
// Please keep in mind not all of this implmentation is
// Final, a lot of it is just Wrapper typing.

/// Login Packet
#[derive(Debug, BinaryStream)]
pub struct Login {
     pub protocol: u8,
     pub request_data: Slice
}

#[derive(Debug, BinaryStream)]
#[repr(u32)]
pub enum PlayStatus {
     // Failed login
     Success = 0,
     FailedClient = 1,
     FailedServer = 2,
     PlayerSpawn = 3,
     InvalidTenant = 4,
     NotEdu = 5,
     EduVanilla = 6,
     ServerFull = 7
}