pub mod tcp;
pub mod udp;
pub mod ethernet;
pub mod ipv4;
pub mod ipv6;

use std::sync::Arc;
use pnet::datalink::{self, Channel};
use std::thread;
use pnet::datalink::NetworkInterface;

use crate::{Parameters, print_program_name, print_error};
use crate::layers;



// All the Available protocols
// NEEDED FOR THE FILTERING, IF SPECIFIED IN ENV ARGUMENTS

#[derive(Clone, PartialEq, Debug)]
pub enum Protocol {
    ETHERNET,
    IPv4,
    IPv6,
    TCP,
    UDP,
    //ICMP,
    //HTTP,
    //HTTPS,
}



// FUNCTION FOR BUILDING THE EASY PARSABLE DATA
// TO LATER BE USED IN THE INDIVIDUAL PROTOCOL FUNCTIONS 
// FOR THE PACKET FILTERING AND DISPLAYING

fn consider_parameters(interfaces : Vec<NetworkInterface>, params : Vec<Parameters>) ->
                    (Vec<NetworkInterface>, Vec<u16>, Vec<String>, Vec<Protocol>) {

    // THE VARIABLES FOR THE CURRENT DATA
    // REPRESENTED IN VECTORS AS THE DATA IS NOT DRAMATICALLY BIG
    // AND THE OVERHEAD WON'T BREAK THE SPEED AND EFFICIENCY

    let mut working_interfaces  : Vec<NetworkInterface> = Vec::new();
    let mut working_ports       : Vec<u16>              = Vec::new();
    let mut working_ips         : Vec<String>           = Vec::new();
    let mut specified_protocols : Vec<Protocol>         = Vec::new();

    // EVERY SINGLE PARAMETER SHOULD BE SEPARATED IN APPROPRIATE VECTOR
    // TO BE LATER USED ON THE FOLLOWING ETAPEE

    for param in params {
        match param {
            Parameters::IpAddress(ips) => {
                working_ips = ips;
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
            },

            Parameters::NoParameter => (),
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

    print_program_name();
    println!("LISTENING ON THE INTERFACES: {:?}", interfaces);

    // TAKING ALL THE PARAMETERS TO WORK WITH INSIDE THE THREAD-LOOP

    let (w_interfaces, w_ports, w_ips, w_prot) = consider_parameters(interfaces, params);

    // WAITING FOR EACH THREAD TO AVOID PREMATURE EXIT AND BUGS

    let mut handles : Vec<std::thread::JoinHandle<_>> = vec![];



    // BECAUSE OF WORKING WITH THREADS, WE NEED TO CLONE THE PROTOCOLS, PORTS AND IP
    // TO ENSURE NO POINTER ISSUES WILL OCCUR DURING THE ANALYSIS

    let w_prot = Arc::new(w_prot);
    let w_ports = Arc::new(w_ports);
    let w_ips = Arc::new(w_ips);

    print_program_name();
    println!("OH, REST! OHREST IS CATCHING THE PACKETS");



    // FOR EACH INTERFACE WE CREATE A THREAD TO LISTEN TO THE PACKETS
    // USUALLY, IT IS A GOOD PRACTICE TO USE THREADS FOR EACH INTERFACE
    // AS THEY ARE INDEPENDENT AND CAN BE ANALYZED SEPARATELY

    for interface in w_interfaces {
        let w_prot  = Arc::clone(&w_prot);
        let w_ports = Arc::clone(&w_ports);
        let w_ips = Arc::clone(&w_ips);

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
                // EVERYTIME WE GET A PACKET, PROCEED
                match rx.next() {

                    // CASE NO ERROR OF READING
                    Ok(packet) => {
                        println!("\n\n\x1b[1mPACKET #{}\x1b[0m, INTERFACE: {}", 
                            packet_id, interface.name);

                        // CHECKING THE PACKET
                        // IT IT MATCHES THE PROTOCOLS
                        // PRINT IT OUT

                        layers::check_all_layers(packet, w_prot.to_vec(), 
                                                w_ips.to_vec(), w_ports.to_vec())
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
        let interfaces = datalink::interfaces();
        let params = vec![Parameters::Interface(vec!["lo".to_string()]),
                          Parameters::Port(vec![80, 32, 1]),
                          Parameters::IpAddress(vec!["127.0.0.1".to_string()])];
        let (w_ints, w_ports, w_ips, w_protocols) = consider_parameters(interfaces, params);

        assert_eq!(w_ints.len(), 1);
        assert_eq!(w_ports.len(), 3);
        assert_eq!(w_ips.len(), 1);
        assert_eq!(w_protocols.len(), 0);
    }

    #[test]
    fn test_find_packets_alone() {
        let params = vec![Parameters::Interface(vec!["wlo1".to_string()]),
                          Parameters::Protocol(vec![Protocol::UDP])];

        find_packets(params);
    }
}
