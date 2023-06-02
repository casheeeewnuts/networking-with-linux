use std::num::ParseIntError;

use pnet::{datalink, packet};
use pnet::datalink::{MacAddr, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::Packet;

use ethernet_echo_protocol::{EthernetEchoProtocolPacket, MessageTypes, MutableEthernetEchoProtocolPacket};

fn main() {
    let interface_name = std::env::args().nth(1).expect("network interface was not provided!");
    let dest_addr = std::env::args().nth(2)
        .ok_or("destination is required!".to_string())
        .and_then(|addr| parse_and_into_mac_addr(&addr))
        .unwrap();

    let message = std::env::args().nth(3).unwrap_or("".to_string());

    let target_interface = datalink::interfaces()
        .into_iter()
        .filter(|iface: &NetworkInterface| iface.name == interface_name)
        .next()
        .unwrap();

    let (mut tx, mut rx) = match datalink::channel(&target_interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("could not listen the interface: {}", interface_name),
        Err(e) => panic!("{}", e),
    };

    let mut buf = Vec::new();
    buf.resize(14 + 4 + message.len(), 0);

    let packet = build_packet(target_interface.mac.unwrap(), dest_addr, &message, &mut buf);

    tx.send_to(packet.packet(), Some(target_interface)).unwrap().expect("an error occurred");

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                let eep_packet = EthernetEchoProtocolPacket::new(packet.payload());

                match eep_packet {
                    Some(eep_packet) => {
                        println!("{} bytes from {}. message={}", eep_packet.packet().len(), packet.get_source() ,std::str::from_utf8(eep_packet.payload()).unwrap());
                        break;
                    }
                    None => {}
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}

fn build_packet<'a>(src: MacAddr, dest: MacAddr, msg: &str, buf: &'a mut Vec<u8>) -> EthernetPacket<'a> {
    let mut ether_frame = MutableEthernetPacket::new(buf.as_mut_slice()).unwrap();
    let mut eep_buf = Vec::new();
    eep_buf.resize(4 + msg.len(), 0);

    let eep_packet = {
        let mut packet = MutableEthernetEchoProtocolPacket::new(eep_buf.as_mut_slice()).unwrap();

        packet.set_message_type(MessageTypes::Request);
        packet.set_payload(msg.as_bytes());

        packet.consume_to_immutable()
    };

    ether_frame.set_source(src);
    ether_frame.set_destination(dest);
    ether_frame.set_payload(eep_packet.packet());
    ether_frame.consume_to_immutable()
}

fn parse_and_into_mac_addr(addr: &str) -> Result<MacAddr, String> {
    addr.split(":")
        .map(|segment| u8::from_str_radix(segment, 16))
        .collect::<Result<Vec<u8>, ParseIntError>>()
        .map_err(|e| e.to_string())
        .and_then(|segments| {
            if segments.len() != 6 {
                Err("invalid mac-address format!".to_string())
            } else {
                Ok(segments)
            }
        })
        .map(|segments| {
            MacAddr(
                segments[0],
                segments[1],
                segments[2],
                segments[3],
                segments[4],
                segments[5],
            )
        })
}

#[test]
fn build_packet_test() {
    let dst = MacAddr(0, 0, 0, 0, 0, 0);
    let src = MacAddr(0, 0, 0, 0, 0, 1);
    let msg = "";
    let mut buf: Vec<u8> = vec![];

    buf.resize(14 + 4 + msg.len(), 0);

    let packet = build_packet(src, dst, msg, &mut buf);

    assert_eq!(packet.get_destination(), dst);
    assert_eq!(packet.get_source(), src);

    let eep_packet = EthernetEchoProtocolPacket::new(packet.payload()).unwrap();
    assert_eq!(eep_packet.get_message_type(), MessageTypes::Request);
    assert_eq!(eep_packet.payload(), msg.as_bytes());

    dbg!(eep_packet);
}