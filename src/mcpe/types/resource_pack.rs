use binary_util::{types::varu64, BinaryIo};

/// Behavior Pack Info
/// This is a type that is used to represent the behavior pack info.
#[derive(BinaryIo, Debug, Clone)]
pub struct BehaviorPackInfoItem {
    /// This is the UUID of the behavior pack.
    /// This is used to uniquely identify the behavior pack.
    pub uuid: String,
    /// This is the version of the behavior pack.
    pub version: String,
    /// The size of the behavior pack. (in bytes)
    pub size: varu64,
    /// The encryption key of the behavior pack.
    pub encryption_key: String,
    /// The subpack name of the behavior pack.
    pub subpack_name: String,
    /// The content identity of the behavior pack.
    pub content_identity: String,
    /// Whether or not the behavior pack contains scripts.
    pub has_scripts: bool,
}

/// Texture Pack Info
pub struct TexturePackInfoItem {
    /// This is the UUID of the texture pack.
    /// This is used to uniquely identify the texture pack.
    pub uuid: String,
    /// This is the version of the texture pack.
    pub version: String,
    /// The size of the texture pack. (in bytes)
    pub size: varu64,
    /// The encryption key of the texture pack.
    pub encryption_key: String,
    /// The subpack name of the texture pack.
    pub subpack_name: String,
    /// The content identity of the texture pack.
    pub content_identity: String,
    /// Whether or not the texture pack contains scripts.
    pub has_scripts: bool,
    /// Whether or not the texture pack has RTX.
    pub has_rtx: bool,
}

#[derive(BinaryIo, Debug, Clone)]
pub struct ResourcePackStackEntry {
    /// The id of the pack
    pub id: String,
    /// The version of the resource pack
    pub version: String,
    /// The name of the reosurce pack.
    pub name: String,
}

#[derive(BinaryIo, Debug, Clone)]
#[repr(u8)]
pub enum ResourcePackStackEntryType {
    /// This is a behavior pack.
    /// More than likely...
    Addon,
    /// Cached pack
    Cached,
    CopyProtected,
    Behavior,
    PersonaPiece,
    Resources,
    SkinPack,
    WorldTemplate,
}
