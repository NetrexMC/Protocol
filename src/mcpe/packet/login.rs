use binary_util::BinaryIo;

#[derive(BinaryIo, Debug, Clone)]
pub struct Login {
    /// This is the protocol version of the client connecting.
    /// You can verify if this version is allowed by using `mcpe::version_within_current_patch(pk.version)`.
    pub version: i32,
    /// This is the client chain data.
    /// For sake of myself i'm not going to document this.
    pub chain_data: Vec<u8>,
}
