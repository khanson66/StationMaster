use std::cmp::Ordering;
use netstat2::*;

struct SocketInfo {
    local_port : u32
}
// for checking system processes and just extracting the used ports regardless of state
pub fn get_used_port() -> Vec<u32> {
    
    let mut sockets = get_sockets(AddressFamilyFlags::IPV4);
    let mut sockets6 = get_sockets(AddressFamilyFlags::IPV6);
    sockets.append(&mut sockets6);
 
    sockets.sort_by(|a, b| {
        if a.local_port < b.local_port {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut used_ports: Vec<u32> = [].to_vec();
    populate_used_port(&sockets, &mut used_ports);
    used_ports.dedup();
    //for item in used_ports{println!("{}",item) };
    used_ports
}

fn populate_used_port(sockets: &Vec<SocketInfo>,list_port: &mut Vec<u32>) {
    for s in sockets { list_port.push(s.local_port);}
}

fn get_sockets(addr: AddressFamilyFlags) -> Vec<SocketInfo> {
        // get TCP only
        let protos = ProtocolFlags::TCP;//| ProtocolFlags::UDP;
        let iterator = iterate_sockets_info(addr, protos).expect("Failed to get socket information!");
    
        let mut sockets: Vec<SocketInfo> = Vec::new();
    
        for info in iterator {
            let si = match info {
                Ok(si) => si,
                Err(_err) => {
                    println!("Failed to get info for socket!");
                    continue;
                }
            };
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp) => sockets.push(SocketInfo {
                local_port: tcp.local_port as u32
            }),
            _ =>{}
        }
    }sockets
}