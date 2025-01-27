// THE FILE MADE FOR DIFFERENT LAYERS OF TCP/IP STACK

use crate::protocols::Protocol;
use crate::protocols::{ethernet, ipv4, ipv6, tcp, udp};
use pnet::packet::ethernet::EtherType;
use pnet::packet::ethernet::EtherTypes;

#[derive(Clone)]
pub enum UpperProtocol {
    Layer1(EtherType),
    Layer2(u8),
    Layer3((u16, u16)),
}

// RETURNING THE FUNCTION ACCORDING TO THE PROTOCOL REQUESTED WITHIN THE LAYER
fn return_check_and_get(protocol : &Protocol) 
                                -> fn(&[u8]) -> Option<(UpperProtocol, Vec<u8>)> {

    match protocol {
        Protocol::ETHERNET => ethernet::check_and_get_next_layer,
        Protocol::IPv4     => ipv4::check_and_get_next_layer,
        Protocol::IPv6     => ipv6::check_and_get_next_layer,
        Protocol::TCP      => tcp::check_and_get_next_layer,
        Protocol::UDP      => udp::check_and_get_next_layer,
    }
}

fn return_print_output(protocol : &Protocol) -> fn(&[u8]) {
    match protocol {
        Protocol::ETHERNET => ethernet::print_output,
        Protocol::IPv4     => ipv4::print_output,
        Protocol::IPv6     => ipv6::print_output,
        Protocol::TCP      => tcp::print_output,
        Protocol::UDP      => udp::print_output,
    }
}

fn check_protocol(protocol : Protocol, packet : &[u8]) 
                                -> Option<(Protocol, UpperProtocol, Vec<u8>)> {


    if let Some(protocol_data) = return_check_and_get(&protocol)(packet) {
        // In case, that was the last layer
        // We can print the output
        // And return the protocol
        return_print_output(&protocol)(packet);
        return Some((protocol, protocol_data.0, protocol_data.1));
    }

    None
}

pub fn check_network_access_layer(packet : Vec<u8>) 
                                  -> Option<(Protocol, UpperProtocol, Vec<u8>)> {

    let packet_array = packet.as_slice();

    // ETHERNET
    if let Some(res) = check_protocol(Protocol::ETHERNET, packet_array) {
        return Some(res);
    }

    None
}

pub fn check_network_layer(packet : Vec<u8>, current_protocol : UpperProtocol) 
                                    -> Option<(Protocol, UpperProtocol, Vec<u8>)> {
    let packet_array = packet.as_slice();

    // THERE ARE TWO PROTOCOLS SUPPORTED BY THE PROGRAM
    // IPv4 AND IPv6
    match current_protocol {
        UpperProtocol::Layer1(EtherTypes::Ipv4) => {
            if let Some(res) = check_protocol(Protocol::IPv4, packet_array) {
                return Some(res);
            }
        },
        UpperProtocol::Layer1(EtherTypes::Ipv6) => {
            if let Some(res) = check_protocol(Protocol::IPv6, packet_array) {
                return Some(res);
            }
        },
        _ => (),
    }

    None
}

pub fn check_transport_layer(packet : Vec<u8>, current_protocol : UpperProtocol) 
                                      -> Option<(Protocol, UpperProtocol, Vec<u8>)> {
    let packet_array = packet.as_slice();

    // TCP AND UDP ARE SUPPORTED

    match current_protocol {
        UpperProtocol::Layer2(6) => {
            if let Some(res) = check_protocol(Protocol::TCP, packet_array) {
                return Some(res);
            }
        },
        UpperProtocol::Layer2(17) => {
            if let Some(res) = check_protocol(Protocol::UDP, packet_array) {
                return Some(res);
            }
        },
        _ => (),
    }

    None
}

pub fn check_application_layer(packet : Vec<u8>, current_protocol : UpperProtocol) 
                                        -> Option<(Protocol, UpperProtocol, Vec<u8>)> {
    let packet_array = packet.as_slice();

    // APPLICATION LAYER IS NOT IMPLEMENTED YET
    None
}

pub fn check_all_layers(packet : &[u8], ips : Vec<String>, ports : Vec<u16>) -> Option<(Protocol, Vec<u8>)> {
    let packet = packet.to_vec();

    if let Some(res_access) = check_network_access_layer(packet) {

        // IF THE ACCESS LAYER WAS VALID AND THE NEXT LAYER EXISTS
        if let Some(res_network) = check_network_layer(res_access.2.clone(), res_access.1.clone()) {

            // IF THE NETWORK LAYER WAS VALID AND THE NEXT LAYER EXISTS
            if let Some(res_transport) = check_transport_layer(res_network.2.clone(), res_network.1.clone()) {

                // IF THE TRANSPORT LAYER WAS VALID AND THE NEXT LAYER EXISTS
                if let Some(res_app) = check_application_layer(res_transport.2.clone(), res_transport.1.clone()) {
                    return Some((res_app.0, res_app.2));
                }
                return Some((res_transport.0, res_transport.2));
            }
            return Some((res_network.0, res_network.2));
        }
        return Some((res_access.0, res_access.2));
    }
    None
    
}
