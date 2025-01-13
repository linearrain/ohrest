pub mod tcp;

use std::sync::Arc;
use pnet::datalink::{self, Channel};
use std::thread;
use pnet::datalink::NetworkInterface;

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
        Protocol::TCP     => tcp::check_and_get(&packet),
        Protocol::UDP     => (),// check_udp(),
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

fn push_all_protocols(protocols : &mut Vec<Protocol>) {
    protocols.push(Protocol::IPv4);
    protocols.push(Protocol::IPv6);
    protocols.push(Protocol::TCP);
    protocols.push(Protocol::UDP);
    protocols.push(Protocol::ICMP);
    protocols.push(Protocol::HTTP);
    protocols.push(Protocol::HTTPS);
}

fn consider_parameters(interfaces : Vec<NetworkInterface>, params : Vec<Parameters>) ->
                           (Vec<NetworkInterface>, Vec<u16>, Vec<String>, Vec<Protocol>) {

    let mut working_interfaces  : Vec<NetworkInterface> = Vec::new();
    let mut working_ports       : Vec<u16>              = Vec::new();
    let mut working_ips         : Vec<String>           = Vec::new();
    let mut working_protocols   : Vec<Protocol>         = Vec::new();

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
                working_protocols = protocols;
            }
        }
    }

    // If no interfaces are specified, use all interfaces

    if working_interfaces.is_empty() {
        working_interfaces = interfaces;
    }

    if working_protocols.is_empty() {
        push_all_protocols(&mut working_protocols);
    }

    (working_interfaces, working_ports, working_ips, working_protocols)
}

fn find_packets(params: Vec<Parameters>) {
    let interfaces = datalink::interfaces();

    let (w_interfaces, _w_ports, _w_ips, w_prot) = consider_parameters(interfaces, params);

    let mut handles : Vec<std::thread::JoinHandle<_>> = vec![];

    let w_prot = Arc::new(w_prot);

    for interface in w_interfaces {
        let w_prot = Arc::clone(&w_prot);

        let handle = thread::spawn(move || {
            let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
                Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
                Ok(_) => panic!("Unhandled channel type"),
                Err(e) => panic!("Error: {}", e),
            };

            loop {
                match rx.next() {
                    Ok(packet) => {
                        for protocol in &*w_prot {
                            check_protocol(protocol.clone(), packet);
                        }
                    },
                    Err(e) => {
                        eprintln!("Error while reading packet: {}", e);
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

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

    #[test]
    fn test_find_packets_alone() {
        let params = vec![Parameters::Interface(vec!["lo".to_string()])];

        find_packets(params);
    }

    #[test]
    fn test_find_packets_all_interfaces() {
        let params = vec![];

        find_packets(params);
    }
}
