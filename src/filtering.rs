use crate::Parameters;

pub trait NetworkLayer {
    // Getting the source and destination of the packet
    fn get_source_dest(&self) -> (String, String);
}

pub fn check_ips<T>(packet : &T, ips : Parameters) -> bool 
where T: NetworkLayer, {
    match ips {
        Parameters::IpAddress(ips) => {
            if ips.is_empty() {
                return true;
            }

            let (source, dest) = packet.get_source_dest();
            if ips.contains(&source) || ips.contains(&dest) {
                return true;
            }
        },
        Parameters::NoParameter => return true,
        _ => panic!("CODE ERROR: IPS ARE ONLY ALLOWED IN NETWORK LAYER"),
    }
    false
}

pub fn check_ports(source : u16, dest : u16, p : Parameters) -> bool {
    match p {
        Parameters::Port(needed_ports) => {
            if needed_ports.is_empty() {
                return true;
            }

            needed_ports.iter().any(|port| {
                *port == source || *port == dest
            });
        },
        Parameters::NoParameter => return true,
        _ => panic!("CODE ERROR: PORTS ARE ONLY ALLOWED IN TRANSPORT LAYER"),
    }

    false
}
