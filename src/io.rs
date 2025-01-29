use crate::{Parameters, print_error};
use crate::protocols::Protocol;

use std::net::IpAddr;

fn incr_and_not_exceed(position : &mut usize, args : &[String]) -> bool {
    *position += 1;

    if *position >= args.len() {
        return false;
    }

    true
}

fn protocols_parse(args : &[String], position : &mut usize) -> Vec<Protocol> {
    let mut protocols : Vec<Protocol> = vec![];

    if !incr_and_not_exceed(position, args) {
        print_error();
        println!("ERROR: NO PROTOCOLS SPECIFIED");
        return protocols;
    }

    while !args[*position].starts_with("-") {
        match args[*position].as_str() {
            "ipv4" => protocols.push(Protocol::IPv4),
            "ipv6" => protocols.push(Protocol::IPv6),
            "tcp"  => protocols.push(Protocol::TCP),
            "udp"  => protocols.push(Protocol::UDP),
            _ => {
                print_error(); 
                println!("INVALID PROTOCOL '{}'", args[*position]);
            }
        }

        if !incr_and_not_exceed(position, args) {
            return protocols;
        }
    }

    protocols
}

fn ip_parse(args : &[String], position : &mut usize) -> Vec<String> {
    let mut ips : Vec<String> = vec![];

    if !incr_and_not_exceed(position, args) {
        print_error();
        println!("NO IPs SPECIFIED");
        return ips;
    }

    while !args[*position].starts_with("-") {
        if let Ok(ip) = args[*position].parse::<IpAddr>() {
            ips.push(ip.to_string());
        }
        else {
            print_error();
            println!("INVALID IP '{}'", args[*position]); 
        }

        if !incr_and_not_exceed(position, args) {
            return ips;
        }
    }

    ips
}

fn port_parse(args : &[String], position : &mut usize) -> Vec<u16> {
    let mut ports : Vec<u16> = vec![];

    if !incr_and_not_exceed(position, args) {
        print_error();
        println!("NO PORTS SPECIFIED");
        return ports;
    }

    while !args[*position].starts_with("-") {
        if let Ok(port) = args[*position].parse::<u16>() {
            ports.push(port);

            if !incr_and_not_exceed(position, args) {
                return ports;
            }
        }
        else {
            print_error();
            println!("INVALID PORT '{}'", args[*position]);
        }

        if !incr_and_not_exceed(position, args) {
            return ports;
        }
    }

    ports
}

pub fn interpret_parameters(args : &[String]) -> Vec<Parameters> {
    let mut parameters : Vec<Parameters> = Vec::new();

    let mut position = 0;

    while position < args.len() {
        match args[position].as_str() {
            "-p"  | "--protocol" => parameters.push(Parameters::Protocol(protocols_parse
                                                                (args, &mut position))),
            "-i"  | "--ip"       => parameters.push(Parameters::IpAddress(ip_parse
                                                                (args, &mut position))),
            "-pt" | "--port"     => parameters.push(Parameters::Port(port_parse
                                                                (args, &mut position))),
            _ => position += 1
        }
    }

    parameters
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easy_input() {
        let args = vec!["-p".to_string(), "ipv6".to_string(),
                        "-pt".to_string(), "172".to_string()];

        let params = interpret_parameters(&args);

        println!("{:?}", params);

        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_no_input() {
        let args = vec!["-p".to_string(), "-pt".to_string()];

        let params = interpret_parameters(&args);

        println!("{:?}", params);

        assert_eq!(params.len(), 2);
    }
}
