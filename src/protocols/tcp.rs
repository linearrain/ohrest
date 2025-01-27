use pnet::packet::Packet;
use crate::{print_program_name, get_color};
use crate::layers::UpperProtocol;

// ETH II -> IPv4 -> TCP

// TCP IS A CONNECTION-ORIENTED PROTOCOL, WHICH:
// IS BASED ON IPV4
// MEANING IT MUST BE A VALID ETHERNET II (WHICH IS CHECKED IN THE MAIN FUNCTION)
// AND IT MUST BE A VALID IPV4 PACKET (CHECKING RIGHT BELOW)

pub fn check_and_get_next_layer(packet : &[u8]) -> Option<(UpperProtocol, Vec<u8>)> {
    if let Some(tcp) = pnet::packet::tcp::TcpPacket::new(packet) {
        return Some((UpperProtocol::Layer3((tcp.get_destination(), tcp.get_source())),
                tcp.payload().to_vec()))
    }

    None
}

pub fn print_output(packet : &[u8]) {
    let packet = pnet::packet::tcp::TcpPacket::new(packet).unwrap();
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
