use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;


// UDP PROTOCOL IS LAYER 4, MEANING, THAT:
// IT MUST BE A VALID IPV4 PACKET (CHECKING RIGHT BELOW)

pub fn check_and_get(packet : &[u8]) {
    // IN CASE THE ETHERNET HEADER IS VALID, WE CAN CHECK THE IPV4 HEADER
    if let Some(ether) = EthernetPacket::new(packet) {
        if let Some(ipv4_packet) = Ipv4Packet::new(ether.payload()) { 
            if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
                print_output(udp_packet);
            }
        }
    }
}

fn print_output(packet : pnet::packet::udp::UdpPacket) {
    println!("TYPE: UDP PACKET");
    println!("Source port: {}   Destination port: {}", packet.get_source(), packet.get_destination());
    println!("Length: {}   Checksum: {}", packet.get_length(), packet.get_checksum());
}
