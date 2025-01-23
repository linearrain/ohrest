// IPv4 Packet demands a valid Ethernet Frame
// Ethernet module already checks it, so we getting there after this check

use pnet::packet::Packet;
use pnet::packet::ipv4::Ipv4Packet;
use crate::print_program_name;

pub fn check_and_get_ipv4(packet : &[u8]) -> Option<Vec<u8>> {
    if let Some(ipv4_packet) = Ipv4Packet::new(packet) {
        return Some(ipv4_packet.payload().to_vec());
    }

    None
}

pub fn print_output(packet : Ipv4Packet) {
    print_program_name();

    println!("IPV4 INFORMATION");
    
    println!("Version: {}   IHL: {}   Type Of Service: {} (DSCP)   QoS: {} (ECN)",
             packet.get_version(), packet.get_header_length(), packet.get_dscp(), 
             packet.get_ecn());

    println!("Total length: {}   ID: {}   Flags: {}   Fragment offset: {}",
             packet.get_total_length(), packet.get_identification(), 
             packet.get_flags(), packet.get_fragment_offset());

    println!("Time to live: {}   Protocol: {}   Header checksum: {}",
             packet.get_ttl(), packet.get_next_level_protocol(), packet.get_checksum());

    println!("Source: {}   Destination: {}", packet.get_source(), packet.get_destination());
    println!("Options: {:?}", packet.get_options());
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

        check_and_get_ipv4(&packet);
    }
}
