extern crate ethernet_echo_protocol;

use std::env;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{MutablePacket, Packet};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};

use ethernet_echo_protocol::{EthernetEchoProtocolPacket, MessageTypes, MutableEthernetEchoProtocolPacket};

fn main() {
    let interface_name = env::args().nth(1).expect("network interface name not provided!");

    let interfaces = datalink::interfaces();
    let target_interface = interfaces.into_iter()
        .filter(|iface: &NetworkInterface| iface.name == interface_name)
        .next()
        .unwrap();
    let target_interface_mac_addr = target_interface.mac.unwrap();

    let (mut tx, mut rx) = match datalink::channel(&target_interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!(),
        Err(e) => panic!("{}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();

                if packet.get_destination() != target_interface_mac_addr {
                    continue;
                }

                println!("{} ---> {}: type={}", packet.get_source(), packet.get_destination(), packet.get_ethertype());

                let eep_packet = EthernetEchoProtocolPacket::new(packet.payload()).unwrap();

                if eep_packet.get_message_type() == MessageTypes::Request {
                    tx.build_and_send(1, packet.packet().len(),
                                      &mut |raw_packet| {
                                          let mut new_packet = MutableEthernetPacket::new(raw_packet).unwrap();
                                          let mut buf = [0u8; 4];
                                          let eep_packet = {
                                              let mut eep_packet = MutableEthernetEchoProtocolPacket::new(&mut buf).unwrap();
                                              eep_packet.set_message_type(MessageTypes::Response);

                                              eep_packet
                                          };

                                          new_packet.clone_from(&packet);

                                          new_packet.set_source(packet.get_destination());
                                          new_packet.set_destination(packet.get_source());
                                          new_packet.set_payload(eep_packet.packet());
                                      },
                    );
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
