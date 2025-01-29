// THE FILE MADE FOR DIFFERENT LAYERS OF TCP/IP STACK

use crate::protocols::Protocol;
use crate::protocols::{ethernet, ipv4, ipv6, tcp, udp};
use crate::Parameters;

use pnet::packet::ethernet::EtherType;
use pnet::packet::ethernet::EtherTypes;

use crate::print_error;

#[derive(Clone, Debug)]
pub enum UpperProtocol {
    Layer1(EtherType),
    Layer2(u8),
    Layer3((u16, u16)),
}

// RETURNING THE FUNCTION ACCORDING TO THE PROTOCOL REQUESTED WITHIN THE LAYER

fn return_print_output(protocol : &Protocol) -> fn(Vec<u8>) {
    match protocol {
        Protocol::ETHERNET => ethernet::print_output,
        Protocol::IPv4     => ipv4::print_output,
        Protocol::IPv6     => ipv6::print_output,
        Protocol::TCP      => tcp::print_output,
        Protocol::UDP      => udp::print_output,
    }
}

pub fn check_network_access_layer(packet : Vec<u8>) 
                                  -> Option<(Protocol, UpperProtocol, Vec<u8>)> {

    let packet_array = packet.as_slice();

    // ETHERNET
    if let Some(res) = ethernet::check_and_get_next_layer(packet_array, 
                                                        Parameters::NoParameter) {
        return Some((Protocol::ETHERNET, res.0, res.1));
    }

    None
}

pub fn check_network_layer(packet : Vec<u8>, current_protocol : UpperProtocol, 
                           ips : &Vec<String>)
                                    -> Option<(Protocol, UpperProtocol, Vec<u8>)> {
    let packet_array = packet.as_slice();

    // THERE ARE TWO PROTOCOLS SUPPORTED BY THE PROGRAM
    // IPv4 AND IPv6

    match current_protocol {
        UpperProtocol::Layer1(EtherTypes::Ipv4) => {
            if let Some(res) = ipv4::check_and_get_next_layer(packet_array, 
                                                    Parameters::IpAddress(ips.to_vec())) {
                return Some((Protocol::IPv4, res.0, res.1));
            }
        },
        UpperProtocol::Layer1(EtherTypes::Ipv6) => {
            if let Some(res) = ipv6::check_and_get_next_layer(packet_array, 
                                                    Parameters::IpAddress(ips.to_vec())) {
                return Some((Protocol::IPv6, res.0, res.1));
            }
        },
        _ => (),
    }

    None
}

pub fn check_transport_layer(packet : Vec<u8>, current_protocol : UpperProtocol,
                             ports : &Vec<u16>) 
                                      -> Option<(Protocol, UpperProtocol, Vec<u8>)> {
    let packet_array = packet.as_slice();

    // TCP AND UDP ARE SUPPORTED

    match current_protocol {
        UpperProtocol::Layer2(6) => {
            if let Some(res) = tcp::check_and_get_next_layer(packet_array, 
                                                    Parameters::Port(ports.to_vec())) {
                return Some((Protocol::TCP, res.0, res.1));
            }
        },
        UpperProtocol::Layer2(17) => {
            if let Some(res) = udp::check_and_get_next_layer(packet_array, 
                                                    Parameters::Port(ports.to_vec())) {
                return Some((Protocol::UDP, res.0, res.1));
            }
        },
        _ => println!("NO TRANSPORT LAYER"),
    }
    
    None
}

pub fn check_application_layer(packet : Vec<u8>, _current_protocol : UpperProtocol) 
                                        -> Option<(Protocol, UpperProtocol, Vec<u8>)> {
    let _packet_array = packet.as_slice();

    // APPLICATION LAYER IS NOT IMPLEMENTED YET
    None
}



// THE STRUCTURE, WHICH REPRESENTS THE CONCRETE LAYER OF THE PACKET
// IT HOLDS THE PROTOCOL AND THE DATA, WHICH CORRESPONDS TO THE LAYER
// TO CORRECTLY LATER CALL ALL THE PRINT FUNCTIONS AFTER CHECKING

#[derive(Debug)]
struct Layer {
    protocol : Protocol,
    data : Vec<u8>,
}

impl Layer {
    fn create(protocol : Protocol, data : Vec<u8>) -> Layer {
        Layer { protocol, data }
    }

    fn contains(&self, protocol : Protocol) -> bool {
        self.protocol == protocol
    }
}

fn print_needed(layers : &Vec<Layer>) {
    for layer in layers {
        return_print_output(&layer.protocol)(layer.data.clone());
    }
}

pub fn check_all_layers(packet_id : usize, int_name : &str, packet : &[u8], 
                        protocols : Vec<Protocol>, ips : Vec<String>, ports : Vec<u16>) {

    let packet            : Vec<u8>    = packet.to_vec();
    let mut passed_layers : Vec<Layer> = Vec::new();

    // IF THE ACCESS LAYER EXISTS AND IS VALID
    let packet_cl = packet.clone();
    if let Some(res_access) = check_network_access_layer(packet) {
        passed_layers.push(Layer::create(res_access.0, packet_cl.clone()));

        // IF THE ACCESS LAYER WAS VALID AND THE NEXT LAYER EXISTS
        if let Some(res_network) = check_network_layer(res_access.2.clone(), 
                                                    res_access.1.clone(), &ips) {

            passed_layers.push(Layer::create(res_network.0, res_access.2.clone()));
            
            // IF THE NETWORK LAYER WAS VALID AND THE NEXT LAYER EXISTS
            if let Some(res_transport) = check_transport_layer(res_network.2.clone(), 
                                                    res_network.1.clone(), &ports) {

                passed_layers.push(Layer::create(res_transport.0, res_network.2.clone()));

                // IF THE TRANSPORT LAYER WAS VALID AND THE NEXT LAYER EXISTS
                if let Some(res_app) = check_application_layer(res_transport.2.clone(), 
                                                        res_transport.1.clone()) {
                    passed_layers.push(Layer::create(res_app.0, 
                                       res_transport.2.clone()));

                }
            }
        }
    }


    if (passed_layers.len() < 2 && !ips.is_empty()) || // IF THERE IS NO NETWORK LAYER, BUT IPs
                                                           // ARE SPECIFIED
       (passed_layers.len() < 3 && !ports.is_empty())  // IF THERE IS NO TRANSPORT LAYER, BUT PORTS
                                                           // ARE SPECIFIED
    {
            return;
    }

    for layer in &passed_layers {
        if protocols.contains(&layer.protocol) || protocols.is_empty() {
            println!("\n\n\x1b[1mPACKET #{}\x1b[0m, INTERFACE: {}", 
                            packet_id, int_name);
            print_needed(&passed_layers);
            break;
        }
    }
}
