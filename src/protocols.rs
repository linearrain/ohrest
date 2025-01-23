pub mod tcp;
pub mod udp;
pub mod ethernet;
pub mod ipv4;

use std::sync::Arc;
use pnet::datalink::{self, Channel};
use std::thread;
use pnet::datalink::NetworkInterface;
use crate::{print_program_name, print_error};

#[derive(Clone)]
pub enum Protocol {
    IPv4,
    IPv6,
    TCP,
    UDP,
    ICMP,
    HTTP,
    HTTPS,
    UNKNOWN
}

pub fn check_protocol(protocol : Protocol, packet : &[u8]) {
    match protocol {
        Protocol::IPv4    => (),// check_ipv4(),
        Protocol::IPv6    => (),// check_ipv6(),
        Protocol::TCP     => tcp::check_and_get(packet),
        Protocol::UDP     => udp::check_and_get(packet),
        Protocol::ICMP    => (),// check_icmp(),
        Protocol::HTTP    => (),// check_http(),
        Protocol::HTTPS   => (),// check_https(),
        Protocol::UNKNOWN => println!("[WARNING] Unknown protocol")
    }
}

// FILTERING THE DATA ACCORDING TO THE INPUT

enum Parameters {
    IpAddress(Vec<String>),
    Port(Vec<u16>),
    Interface(Vec<String>),
    Protocol(Vec<Protocol>)
}

fn get_interfaces() -> Vec<NetworkInterface> {
    pnet::datalink::interfaces()
}

fn consider_parameters(interfaces : Vec<NetworkInterface>, params : Vec<Parameters>) ->
                           (Vec<NetworkInterface>, Vec<u16>, Vec<String>, Vec<Protocol>) {

    let mut working_interfaces  : Vec<NetworkInterface> = Vec::new();
    let mut working_ports       : Vec<u16>              = Vec::new();
    let mut working_ips         : Vec<String>           = Vec::new();
    let mut specified_protocols : Vec<Protocol>         = Vec::new();

    for param in params {
        match param {
            Parameters::IpAddress(ip_address) => {
                working_ips = ip_address;
            },
            Parameters::Port(ports) => {
                working_ports = ports;
            },
            Parameters::Interface(interface_names) => {
                working_interfaces = interfaces.iter()
                    .filter(|interface| interface_names.contains(&interface.name))
                    .map(|interface| interface.clone())
                    .collect();
            },
            Parameters::Protocol(protocols) => {
                specified_protocols = protocols;
            }
        }
    }

    // If no interfaces are specified, use all interfaces

    if working_interfaces.is_empty() {
        working_interfaces = interfaces;
    }

    (working_interfaces, working_ports, working_ips, specified_protocols)
}

fn find_packets(params: Vec<Parameters>) {
    // GETTING THE DEVICES AVAILABLE FOR THE PROGRAM
    let interfaces = datalink::interfaces();

    let (w_interfaces, _w_ports, _w_ips, w_prot) = consider_parameters(interfaces, params);

    // WAITING FOR EACH THREAD TO AVOID PREMATURE EXIT AND BUGS

    let mut handles : Vec<std::thread::JoinHandle<_>> = vec![];



    // BECAUSE OF WORKING WITH THREADS, WE NEED TO CLONE THE PROTOCOLS
    // TO ENSURE NO POINTER ISSUES WILL OCCUR DURING THE ANALYSIS

    let w_prot = Arc::new(w_prot);

    print_program_name();
    println!("OH, REST! OHREST IS CATCHING THE PACKETS");



    // FOR EACH INTERFACE WE CREATE A THREAD TO LISTEN TO THE PACKETS
    // USUALLY, IT IS A GOOD PRACTICE TO USE THREADS FOR EACH INTERFACE
    // AS THEY ARE INDEPENDENT AND CAN BE ANALYZED SEPARATELY

    for interface in w_interfaces {
        let w_prot = Arc::clone(&w_prot);

        // CREATING A THREAD FOR EACH INTERFACE

        let handle = thread::spawn(move || {
            // OPENING A CHANNEL FOR THE INTERFACE
            // IT WILL LISTEN FOR 16-BIT DATA
            // WHICH LATER WOULD BE CONVERTED TO PACKETS
            
            let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
                Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
                Ok(_) => panic!("Unhandled channel type"),
                Err(e) => panic!("Error: {}", e),
            };

            // FOR A SAKE OF READABILITY, EACH PACKET GETS A PACKET ID

            let mut packet_id : usize = 0;



            // STARTING THE LISTENING
            // NO PACKET LIMITATION, IT WILL LISTEN UNTIL THE PROGRAM IS STOPPED

            loop {
                match rx.next() {
                    Ok(packet) => {
                        println!("\x1b[1mPACKET #{}\x1b[0m", packet_id);

                        // IN CASE NO PROTOCOLS ARE SPECIFIED, WE CHECK ALL OF THEM
                        if w_prot.is_empty() {

                            continue;
                        }

                        // IF THE PACKETS ARE SPECIFIED, WE CHECK ONLY THEM
                        // CHECKING THE PROTOCOLS USING THE FUNCTIONS
                        // INTENDED FOR EACH OF THEM DIRECTLY
                        // IN CASE OF UNKNOWN PROTOCOL, WE PRINT A WARNING

                        for protocol in &*w_prot {
                            check_protocol(protocol.clone(), packet);
                        }
                    },
                    Err(..) => {
                        print_error();
                        println!("SOME PACKET GOT CORRUPTED EITHER THE PROGRAM TREATS IT AS SUCH");
                    }
                }

                packet_id += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}






// TESTS

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_consider_parameters() {
        let interfaces = get_interfaces();
        let params = vec![Parameters::Interface(vec!["lo".to_string()]),
                          Parameters::Port(vec![80, 32, 1]),
                          Parameters::IpAddress(vec!["127.0.0.1".to_string()])];
        let (w_ints, w_ports, w_ips, w_protocols) = consider_parameters(interfaces, params);

        assert_eq!(w_ints.len(), 1);
        assert_eq!(w_ports.len(), 3);
        assert_eq!(w_ips.len(), 1);
        assert_eq!(w_protocols.len(), 7);
    }

    /*#[test]
    fn test_find_packets_alone() {
        let params = vec![Parameters::Interface(vec!["lo".to_string()])];

        find_packets(params);
    }

    #[test]
    fn test_find_packets_all_interfaces() {
        let params = vec![];

        find_packets(params);
    }*/
}
