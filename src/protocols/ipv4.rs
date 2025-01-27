// IPv4 Packet demands a valid Ethernet Frame
// Ethernet module already checks it, so we getting there after this check

use pnet::packet::Packet;
use pnet::packet::ipv4::Ipv4Packet;
use crate::{print_program_name, get_color};
use crate::layers::UpperProtocol;

pub fn check_and_get_next_layer(packet : &[u8]) -> Option<(UpperProtocol, Vec<u8>)> {
    if packet[0] >> 4 != 4 {
        return None;
    }

    if let Some(ipv4_packet) = Ipv4Packet::new(packet) {
        return Some((UpperProtocol::Layer2(ipv4_packet.get_next_level_protocol().0), 
                ipv4_packet.payload().to_vec()));
    }

    None
}

pub fn print_output(packet : &[u8]) {
    // Getting the Ipv4Packet from the payload
    let packet = Ipv4Packet::new(packet).unwrap();
    print_program_name();

    println!("{}> > > IPV4 INFORMATION{}", get_color(1), get_color(0));

    print!("{}Version:{} {}   {}IHL:{} {}", get_color(2), get_color(0), 
        packet.get_version(), get_color(4), get_color(0), packet.get_header_length());

    println!("   {}Type Of Service:{} {} (DSCP)   {}QoS:{} {} (ECN)", get_color(2),
             get_color(0), packet.get_dscp(), get_color(5), get_color(2), packet.get_ecn());

    println!("Total length: {}   ID: {}   Flags: {}   Fragment offset: {}",
             packet.get_total_length(), packet.get_identification(), 
             packet.get_flags(), packet.get_fragment_offset());

    println!("Time to live: {}   Protocol: {}   Header checksum: {}",
             packet.get_ttl(), packet.get_next_level_protocol(), packet.get_checksum());

    println!("{}Source:{} {}   {}Destination:{} {}", get_color(2), get_color(0), 
        packet.get_source(), get_color(4), get_color(0), packet.get_destination());
    println!("{}Options:{} {:?}", get_color(5), get_color(0), packet.get_options());
}

// TESTS:

#[cfg(test)]

mod test {
    use super::*;

    #[test]

    fn test_check_and_get_ipv4() {
        let packet = [0x45, 0x00, 0x00, 0x28, 0x00, 0x00, 0x40, 0x00, 0x40, 0x06, 
                      0x3c, 0xce, 0x7f, 0x00, 0x00, 0x01, 0x19, 0xc8, 0xc7, 0x66, 
                      0x00, 0x00, 0x00, 0x00, 0x86, 0x07, 0x39, 0xff, 0x50, 0x14, 
                      0x00, 0x00, 0x10, 0x99, 0x00, 0x00];

        check_and_get_next_layer(&packet);
    }
}
