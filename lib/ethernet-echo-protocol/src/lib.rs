use pnet::packet::PrimitiveValues;
use pnet_macros::{packet};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageType(pub u16);

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod MessageTypes {
    use crate::MessageType;

    pub const Request: MessageType = MessageType(0x00);
    pub const Response: MessageType = MessageType(0x08);
}

///                 EthernetEchoProtocol format
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// | message_type |   check_sum   |           padding            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                             |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                             |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                             |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
#[packet]
pub struct EthernetEchoProtocol {
    #[construct_with(u16)]
    pub message_type: MessageType,

    #[payload]
    pub payload: Vec<u8>
}

impl MessageType {
    pub fn new(val: u16) -> MessageType {
        MessageType(val)
    }
}

impl PrimitiveValues for MessageType {
    type T = (u16,);
    fn to_primitive_values(&self) -> (u16,) {
        (self.0,)
    }
}

#[test]
fn ethernet_echo_protocol_packet_header_test() {
    let mut packet = [0u8; 14];
    let mut eep_packet = MutableEthernetEchoProtocolPacket::new(&mut packet[..]).unwrap();

    eep_packet.set_message_type(MessageTypes::Request);
    assert_eq!(eep_packet.get_message_type(), MessageTypes::Request);
}
