use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{MacAddr, NetworkInterface};
use pnet::packet::ethernet::{EtherType, MutableEthernetPacket};
use pnet::packet::Packet;
use std::num::ParseIntError;

fn main() -> std::io::Result<()> {
    let interface_name = std::env::args().nth(1).unwrap();
    let dest_addr =
        parse_mac_addr(&std::env::args().nth(2).unwrap()).expect("Invalid mac-address!");

    let target_interface = datalink::interfaces()
        .into_iter()
        .filter(|iface: &NetworkInterface| iface.name == interface_name)
        .next()
        .unwrap();

    let (mut tx, _) = match datalink::channel(&target_interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("could not listen the interface: {}", interface_name),
        Err(e) => panic!("{}", e),
    };

    let mut packet = [0u8; 14];
    let packet = {
        let mut packet = MutableEthernetPacket::new(&mut packet[..]).unwrap();

        packet.set_destination(dest_addr);
        packet.set_source(target_interface.mac.unwrap());
        packet.set_ethertype(EtherType(0x9201));

        packet
    };

    tx.send_to(packet.packet(), Some(target_interface)).unwrap()
}

fn parse_and_into_mac_addr(addr: &str) -> Result<MacAddr, String> {
    addr.split(":")
        .map(|segment| u8::from_str_radix(segment, 16))
        .collect::<Result<Vec<u8>, ParseIntError>>()
        .map_err(|e| e.to_string())
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
