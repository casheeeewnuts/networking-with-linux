use pnet::packet::PrimitiveValues;
use pnet_macros::{packet};
use pnet_macros;
use pnet_macros_support::packet::Packet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageType(pub u8);

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
/// | message_type |                     padding                  |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                           payload                           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                             |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                             |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
#[packet]
pub struct EthernetEchoProtocol {
    #[construct_with(u8)]
    pub message_type: MessageType,

    #[length = "3"]
    pub padding: Vec<u8>,

    #[payload]
    pub payload: Vec<u8>
}

impl MessageType {
    pub fn new(val: u8) -> MessageType {
        MessageType(val)
    }
}

impl PrimitiveValues for MessageType {
    type T = (u8,);

    #[inline]
    fn to_primitive_values(&self) -> (u8,) {
        (self.0,)
    }
}

#[test]
fn ethernet_echo_protocol_packet_header_test() {
    let mut packet = [0u8; 4 + 6];
    let mut eep_packet = MutableEthernetEchoProtocolPacket::new(&mut packet[..]).unwrap();

    eep_packet.set_message_type(MessageTypes::Response);
    assert_eq!(eep_packet.get_message_type(), MessageTypes::Response);

    eep_packet.set_payload("hello!".as_bytes());
    dbg!(std::string::String::from_utf8(eep_packet.payload().to_vec()).unwrap());
    dbg!(packet);
}
