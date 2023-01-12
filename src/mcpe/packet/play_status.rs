use binary_utils::*;
use super::packet::*;
use crate::packet_id;

#[derive(Debug, Clone, BinaryStream, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum PlayStatus {
    /// The client can successfully join the game
    Success = 0,
    /// The client has an outdated protocol version and may not join the server.
    FailedClient = 1,
    /// The client version is not supported and the server version is outdated when compared to the client.
    FailedServer = 2,
    /// The client can join and is being spawned into the world.
    PlayerSpawn = 3,
    /// Caused when the client does not have permission to join the server.
    /// Caused on EDU edition servers.
    InvalidTenant = 4,
    /// The client tried connect to the server with Education Edition, but the server is not an EDU edition server.
    NotEdu = 5,
    /// The client tried to connect to the server with a different edition than the server.
    EduVanilla = 6,
    /// The client tried to join the server while it was full.
    ServerFull = 7,
}
packet_id!(PlayStatus, 0x02);