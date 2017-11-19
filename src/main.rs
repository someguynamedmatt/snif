extern crate pnet_datalink;
extern crate ipnetwork;
extern crate pnet;

use std::env;
use std::io::{self, Write};
use std::process;
use ipnetwork::IpNetwork;
use pnet::packet::ethernet::{EthernetPacket};
use pnet::datalink::Channel::Ethernet;

fn main() {
    let interfaces = pnet_datalink::interfaces();
    let iface_arg = match env::args().nth(1) {
        Some(i) => i,
        None => {
            writeln!(io::stderr(), "Network interface name not supplied!").unwrap();
            process::exit(1);
        },
    };

    let interface_match = |iface: &pnet_datalink::NetworkInterface| iface.name == iface_arg;
    let interface = interfaces.into_iter().filter(interface_match).next().unwrap();

    println!("===================");
    println!("IP:");
    for ip in &interface.ips {
        match ip {
            &IpNetwork::V4(a) => println!("   IPv4: {}", a.to_string()),
            &IpNetwork::V6(a) => println!("   IPv6: {}", a.to_string()),
        }
    }
    println!("-------------------");
    let mac_addr = interface.mac.map(|mac| mac.to_string()).expect("???");
    println!("Mac Addr:");
    println!("   {}", mac_addr);
    println!("===================");

    let (mut tx, mut rx) = match pnet_datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type!"),
        Err(e) => panic!("An error occurred when creating the datalink: {}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                println!("Packet info:");
                println!("   From: {:?}", packet.get_source());
                println!("   To: {:?}", packet.get_destination());
            },
            Err(e) => {
                panic!("ERROR WITH PACKET!!!");
            }
        }
    }
}
