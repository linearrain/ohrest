use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;
use pnet::packet::udp::UdpPacket;


// UDP PROTOCOL IS LAYER 4, MEANING, THAT:
// IT MUST BE A VALID IPV4 PACKET (CHECKING RIGHT BELOW)

fn check_and_get(packet : &[u8]) {
    if let Some(ether) = EthernetPacket::new(packet) {
        if let Some(ipv4_packet) = pnet::packet::ipv4::Ipv4Packet::new(packet) { 
            if let Some(udp_packet) = pnet::packet::udp::UdpPacket::new(ether.payload()) {
                print_output(udp_packet);
            }
        }
    }
}

fn print_output(packet : pnet::packet::udp::UdpPacket) {
    println!("UDP PACKET");
    println!("Source port: {}   Destination port: {}", packet.get_source(), packet.get_destination());
    println!("Length: {}   Checksum: {}", packet.get_length(), packet.get_checksum());
}
