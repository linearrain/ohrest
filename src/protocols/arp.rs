use crate::Parameters;
use pnet::packet::arp::ArpPacket;

use crate::layers::UpperProtocol;
use crate::{print_program_name, get_color};
use crate::filtering::check_arp_operation;

pub fn check_and_get_next_layer(packet : &[u8], params : Parameters) 
                                                        -> Option<(UpperProtocol, Vec<u8>)> {

    if let Some(arp) = ArpPacket::new(packet) {
        if check_arp_operation(arp.get_operation().0, params) {
            return Some((UpperProtocol::Layer2(0), vec![]));
        }
    }

    None 
}

pub fn print_output(packet : Vec<u8>) {
    let packet = ArpPacket::new(&packet).unwrap();

    print_program_name();
    println!("\x1b[1m> > > ARP PACKET\x1b[0m");

    println!("{}Hardware type:{} {}   {}Protocol Type:{} {}   {}Hardware Len:{} {}", 
             get_color(2), get_color(0), packet.get_hardware_type().0,
             get_color(4), get_color(0), packet.get_protocol_type(),
             get_color(5), get_color(0), packet.get_hw_addr_len());

    println!("{}Protocol Len:{} {}   {}Operation:{} {} ", 
             get_color(2), get_color(0), packet.get_protocol_type().0, 
             get_color(4), get_color(0), packet.get_operation().0);  

    println!("{}Sender HW Address:{} {}   {}Sender Protocol Address:{} {}", 
        get_color(2), get_color(0), packet.get_sender_hw_addr(),
        get_color(4), get_color(0), packet.get_sender_proto_addr());

    println!("{}Target HW Address:{} {}   {}Target Protocol Address:{} {}",
        get_color(4), get_color(0), packet.get_target_hw_addr(),
        get_color(5), get_color(0), packet.get_target_proto_addr());
}



#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_check_and_get_next_layer() {
        let packet = [0x00, 0x01, 0x08, 0x00, 0x06, 0x04, 0x00, 0x01, 
                      0x26, 0x57, 0x8e, 0xd6, 0x4a, 0x7a, 0x91, 0xae,
                      0xa1, 0xd3];

        let params = Parameters::NoParameter;

        let res = check_and_get_next_layer(&packet, params);

        assert!(res.is_some());
    }
}
