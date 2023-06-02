use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use std::env;

fn main() {
    let interface_name = env::args().nth(1).expect("network interface name not provided!");

    let interfaces = datalink::interfaces();
    let target_interface = interfaces.iter()
        .filter(|iface: &&NetworkInterface| iface.name == interface_name)
        .next()
        .unwrap();

    let (mut tx, mut rx) = match datalink::channel(target_interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!(),
        Err(e) => panic!("{}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();

                tx.build_and_send(1, packet.packet().len(),
                    &mut |new_packet| {
                        let mut new_packet = MutableEthernetPacket::new(new_packet).unwrap();

                        new_packet.clone_from(&packet);

                        new_packet.set_source(packet.get_destination());
                        new_packet.set_destination(packet.get_source());
                    }
                );
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
