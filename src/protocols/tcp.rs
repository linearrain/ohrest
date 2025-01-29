use pnet::packet::Packet;
use pnet::packet::tcp::TcpPacket;

use crate::{print_program_name, get_color};
use crate::layers::UpperProtocol;

use crate::Parameters;

use crate::filtering::check_ports;



pub fn check_and_get_next_layer(packet : &[u8], p : Parameters) 
                                            -> Option<(UpperProtocol, Vec<u8>)> {
    if let Some(tcp) = TcpPacket::new(packet) {
        if check_ports(tcp.get_source(), tcp.get_source(), p) {
            return Some((UpperProtocol::Layer3((tcp.get_destination(), tcp.get_source())),
                tcp.payload().to_vec()))
        }
        return Some((UpperProtocol::Layer3((tcp.get_destination(), tcp.get_source())),
                tcp.payload().to_vec()))
    }

    None
}

pub fn print_output(packet : Vec<u8>) {
    let packet = TcpPacket::new(&packet).unwrap();
    print_program_name();

    println!("{}> > > > TCP PACKET{}", get_color(1), get_color(0));
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
