use pnet::packet::Packet;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ethernet::EtherType;

use crate::layers::UpperProtocol;

use crate::{print_program_name, get_color};

pub fn check_and_get_next_layer(packet : &[u8]) -> Option<(UpperProtocol, Vec<u8>)> {
    if let Some(ether) = EthernetPacket::new(packet) {
        return Some((UpperProtocol::Layer1(ether.get_ethertype()), 
                ether.payload().to_vec()));
    }

    None
}

pub fn print_output(packet : &[u8]) {
    let packet = EthernetPacket::new(packet).unwrap();

    print_program_name();
    println!("{}> ETHERNET INFORMATION{}", get_color(1), get_color(0)); 

    println!("{}Destination:{} {}   {}Source:{} {}   {}Type:{} {}   {}Data Length:{} {}", 
             get_color(4), get_color(0), packet.get_destination(), 
             get_color(2), get_color(0), packet.get_source(), 
             get_color(5), get_color(0), packet.get_ethertype(),
             get_color(2), get_color(0), packet.payload().len());
}
