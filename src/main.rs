//! `SNIf`: Simple Network Interface
//!
//! Quickly (and cleanly) check the configurations of your network devices

extern crate pnet_datalink;
extern crate ipnetwork;

use std::env;
use std::io;
use std::io::prelude::*;
use ipnetwork::IpNetwork;

mod lib;
use lib::*;

#[cfg(not(test))]
fn main() {
    let interfaces = pnet_datalink::interfaces();
    let iface_arg = env::args().nth(1).expect("Network interface name not supplied!");

    let interface_match = |iface: &pnet_datalink::NetworkInterface| iface.name == iface_arg;
    let interface = interfaces.into_iter().find(interface_match).unwrap();

    io::stdout().write("===================\n".as_bytes());
    io::stdout().write("IP:\n".as_bytes());
    for ip in &interface.ips {
        match *ip {
            IpNetwork::V4(a) => println!("   IPv4: {}", a),
            IpNetwork::V6(a) => println!("   IPv6: {}", a),
        }
    }

    io::stdout().write("-------------------\n".as_bytes());
    io::stdout().write("Mac Addr:\n".as_bytes());
    let mac_address = interface.mac.expect("???");
    io::stdout().write(mac_address.to_string().as_bytes());
    io::stdout().write("\n".as_bytes());
    io::stdout().write("===================\n".as_bytes());

    /*
    let (_, mut rx) = match pnet_datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type!"),
        Err(e) => panic!("An error occurred when creating the datalink: {}", e)
    };

    loop {
        let packet = EthernetPacket::new(rx.next().expect("ERROR WITH PACKET!!!"));
        handle_packet("wlp4s0", &packet.unwrap());
    }
    */
}
