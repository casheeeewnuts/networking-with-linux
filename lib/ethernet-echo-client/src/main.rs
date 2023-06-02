use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{MacAddr, NetworkInterface};
use pnet::packet::Packet;
use pnet::packet::ethernet::{MutableEthernetPacket, EtherType};

fn main() -> std::io::Result<()> {
    let interface_name = std::env::args().nth(1).unwrap();
    // let dest = {
    //     let raw_addr = std::env::args().nth(2).unwrap();
    // };

    let target_interface = datalink::interfaces().into_iter()
        .filter(|iface: &NetworkInterface| iface.name == interface_name)
        .next()
        .unwrap();

    let (mut tx, _) = match datalink::channel(&target_interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!(),
        Err(e) => panic!()
    };

    let mut packet = [0u8; 14];
    let packet = {
        let mut packet = MutableEthernetPacket::new(&mut packet[..]).unwrap();

        packet.set_destination(MacAddr::from([0x02, 0x66, 0x04, 0x0c, 0x18, 0x9b]));
        packet.set_source(target_interface.mac.unwrap());
        packet.set_ethertype(EtherType(0x9201));

        packet
    };

    tx.send_to(packet.packet(), Some(target_interface)).unwrap()
}