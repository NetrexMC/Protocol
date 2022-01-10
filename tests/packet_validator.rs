use binary_utils::*;
use mcpe_protocol::mcpe::*;

/// Tests that login packet is equal to that of the one recieved in packet kind
#[test]
fn test_status() {
    let pk: Packet = PlayStatus::Success.into();
    let buf = pk.parse().unwrap();

    let packet_raw = Packet::compose(&buf[..], &mut 0).unwrap();
    let status: PlayStatus = packet_raw.kind.into();

    assert_eq!(PlayStatus::from(pk.kind), status);
}


#[test]
fn instance_equals_static() {
    assert_eq!(PlayStatus::id(), PlayStatus::Success.get_id());
}