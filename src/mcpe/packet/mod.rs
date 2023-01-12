// This file is messy, im sorry :(
macro_rules! export {
    ($t: ident) => {
        pub mod $t;
        pub use $t::*;
    };
}

export!(packet);
export!(behavior_pack_info);
export!(client_to_server_hs);
export!(disconnect);
export!(login);
export!(play_status);
export!(request_network_settings);
export!(resource_pack_info);
export!(server_to_client_hs);
export!(texture_pack_info);


#[macro_export]
macro_rules! packet_id {
    ($name: ident, $id: literal) => {
        impl PacketId for $name {
            fn id() -> u32 {
                $id
            }
        }
    };
}