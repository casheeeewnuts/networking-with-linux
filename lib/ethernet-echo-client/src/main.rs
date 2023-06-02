use std::env;
use pnet::datalink::{self};
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::MacAddr;
use pnet::packet::Packet;
use pnet::packet::ethernet::{MutableEthernetPacket};
use hex;

fn main() {
    let interface_name = env::args().nth(1).expect("target address does not provided!");
    let raw_mac_addr = env::args().nth(2).expect("target address does not provided!");

    let interface = datalink::linux::interfaces().into_iter()
        .filter(|iface| iface.name == interface_name)
        .next()
        .unwrap();

    let (mut tx, _) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!(),
        Err(e) => panic!("{}", e)
    };

    let mut packet = [0u8; 14];
    let packet: MutableEthernetPacket = {
        let mac_addr = hex::decode(raw_mac_addr).unwrap();
        let mut ethernet_header = MutableEthernetPacket::new(&mut packet[..]).unwrap();

        let dest_addr = MacAddr::new(mac_addr[0], mac_addr[1], mac_addr[2], mac_addr[3], mac_addr[4], mac_addr[5]);
        ethernet_header.set_source(interface.mac.unwrap());

        ethernet_header.set_destination(dest_addr);

        ethernet_header
    };

    tx.send_to(packet.packet(), None);
}