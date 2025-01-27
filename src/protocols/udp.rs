use pnet::packet::Packet;
use pnet::packet::udp::UdpPacket;

use crate::{print_program_name, get_color};
use crate::layers::UpperProtocol;

// UDP PROTOCOL IS LAYER 4, MEANING, THAT:
// IT MUST BE A VALID IPV4 PACKET (CHECKING RIGHT BELOW)

pub fn check_and_get_next_layer(packet : &[u8]) -> Option<(UpperProtocol, Vec<u8>)> {
    if let Some(udp) = UdpPacket::new(packet) {
        return Some((UpperProtocol::Layer3((udp.get_destination(), udp.get_source())),
                udp.payload().to_vec()));
    }

    None
}

pub fn print_output(packet : &[u8]) {
    let packet = pnet::packet::udp::UdpPacket::new(packet).unwrap();

    print_program_name();

    println!("{}> > > > UDP PACKET{}", get_color(1), get_color(0));
    println!("Source port: {}   Destination port: {}", packet.get_source(), packet.get_destination());
    println!("Length: {}   Checksum: {}", packet.get_length(), packet.get_checksum());
}
