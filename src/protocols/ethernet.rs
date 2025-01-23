use pnet::packet::Packet;
use pnet::packet::ethernet::EthernetPacket;

pub fn check_and_get_eth(packet : &[u8]) -> Option<Vec<u8>> {
    if let Some(ether) = EthernetPacket::new(packet) {
        return Some(ether.payload().to_vec());
    }

    None
}

pub fn print_output(packet : EthernetPacket) {
    println!("ETHERNET INFORMATION");

    println!("Source: {}   Destination: {}   Type: {}", packet.get_source(), 
             packet.get_destination(), packet.get_ethertype());
}
