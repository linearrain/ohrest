use pnet::datalink::{self, Channel};
use pnet::packet::Packet;
use pnet::packet::ethernet::EthernetPacket;
use crate::protocols::{Parameters, consider_parameters};
use std::thread;
use crate::print_program_name;

// ETH II -> IPv4 -> TCP

// TCP IS A CONNECTION-ORIENTED PROTOCOL, WHICH:
// IS BASED ON IPV4
// MEANING IT MUST BE A VALID ETHERNET II (WHICH IS CHECKED IN THE MAIN FUNCTION)
// AND IT MUST BE A VALID IPV4 PACKET (CHECKING RIGHT BELOW)

pub fn check_and_get(packet : &[u8]) {
    if let Some(ether) = EthernetPacket::new(packet) {
        if let Some(ipv4_packet) = pnet::packet::ipv4::Ipv4Packet::new(packet) { 
            if let Some(tcp_packet) = pnet::packet::tcp::TcpPacket::new(ether.payload()) {
                print_output(tcp_packet);
            }
        }
    }
}

pub fn print_output(packet : pnet::packet::tcp::TcpPacket) {
    print_program_name();

    println!("TYPE: TCP PACKET");
    println!("Source port: {}   Destination port: {}", packet.get_source(), packet.get_destination());
    println!("Sequence number: {}   Ack number: {}", packet.get_sequence(), packet.get_acknowledgement());
    println!("Flags: {}   Window size: {}   Checksum: {}", packet.get_flags(), 
             packet.get_window(), packet.get_checksum());

    println!("Urgent pointer: {}", packet.get_urgent_ptr());
    
}

//
// TESTS
//

#[cfg(test)]
mod tests {
    //use super::*;
}
