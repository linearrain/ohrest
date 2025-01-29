use pnet::packet::Packet;
use pnet::packet::ipv6::Ipv6Packet;
use crate::{print_program_name, get_color};
use crate::layers::UpperProtocol;
use crate::Parameters;

use crate::filtering::{NetworkLayer, check_ips};

impl NetworkLayer for Ipv6Packet<'_> {
    fn get_source_dest(&self) -> (String, String) {
        (self.get_source().to_string(), 
        self.get_destination().to_string())
    }
}

pub fn check_and_get_next_layer(packet : &[u8], ips: Parameters) 
                                            -> Option<(UpperProtocol, Vec<u8>)> {
    if packet[0] >> 4 != 6 {
        return None
    }

    if let Some(ipv6_data) = Ipv6Packet::new(packet) {
        if check_ips(&ipv6_data, ips) {
            return Some((UpperProtocol::Layer2(ipv6_data.get_next_header().0), 
                ipv6_data.payload().to_vec()));
        }
    }

    None
}

pub fn print_output(packet : Vec<u8>) {
    let packet = Ipv6Packet::new(&packet).unwrap();

    print_program_name();

    println!("{}> > > > IPV6 INFORMATION{}", get_color(1), get_color(0));

    println!("{}Version:{} {}   {}Traffic class:{} {}   {}Flow label:{} {}", 
             get_color(2), get_color(0), packet.get_version(), 
             get_color(4), get_color(0), packet.get_traffic_class(), 
             get_color(5), get_color(0), packet.get_flow_label());

    println!("{}Payload length:{} {}   {}Next header:{} {}   {}Hop limit:{} {}",
             get_color(2), get_color(0), packet.get_payload_length(), 
             get_color(4), get_color(0), packet.get_next_header(), 
             get_color(5), get_color(0), packet.get_hop_limit());

    println!("{}Source:{} {}   \n{}Destination:{} {}", 
             get_color(2), get_color(0), packet.get_source(),
             get_color(4), get_color(0), packet.get_destination()); 
}
