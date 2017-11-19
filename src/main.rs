extern crate pnet_datalink;
extern crate ipnetwork;
extern crate pnet;

use std::env;
use std::io::{self, Write};
use std::process;
use ipnetwork::IpNetwork;
use std::net::IpAddr;
use pnet::packet::Packet;
use pnet::packet::arp::ArpPacket;
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
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
                //println!("Packet info:");
                //println!("{:?}", packet);
                //println!("   From: {:?}", packet.get_source());
                //println!("   To: {:?}", packet.get_destination());
                handle_packet("wlp4s0", &packet);
            },
            Err(e) => {
                panic!("ERROR WITH PACKET!!!");
            }
        }
    }
}

fn handle_packet(interface_name: &str, ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(interface_name, ethernet),
        //EtherTypes::Ipv6 => handle_ipv6_packet(interface_name, ethernet),
        EtherTypes::Arp => handle_arp_packet(interface_name, ethernet),
        _ => {
            println!("Other ethertype")
        }
    }
}

fn handle_arp_packet(interface_name: &str, ethernet: &EthernetPacket) {
    println!("Handle ARP packet!");
    let header = ArpPacket::new(ethernet.payload());
    if let Some(header) = header {
        println!("[{}]: ARP packet: {}({}) > {}({}); operation: {:?}",
        interface_name,
        ethernet.get_source(),
        header.get_sender_proto_addr(),
        ethernet.get_destination(),
        header.get_target_proto_addr(),
        header.get_operation());
    } else {
        println!("[{}]: Malformed ARP Packet", interface_name);
    }
}

fn handle_ipv4_packet(interface_name: &str, ethernet: &EthernetPacket) {
    println!("Handle IPv4 Packet");
    let header = Ipv4Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(interface_name,
                                  IpAddr::V4(header.get_source()),
                                  IpAddr::V4(header.get_destination()),
                                  header.get_next_level_protocol(),
                                  header.payload());
    } else {
        println!("Malformed IPv4 Packet!!!")
    }
}

fn handle_tcp_packet(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8]) {
    let tcp = TcpPacket::new(packet);
    if let Some(tcp) = tcp {
        println!("[{}]: TCP Packet: {}:{} > {}:{}; length: {}", interface_name,
                 source,
                 tcp.get_source(),
                 destination,
                 tcp.get_destination(),
                 packet.len());
    } else {
        println!("Malformed TCP packet");
    }
}

fn handle_transport_protocol(interface_name: &str, source: IpAddr, destination: IpAddr,
                             protocol: IpNextHeaderProtocol, packet: &[u8]) {
    match protocol {
        IpNextHeaderProtocols::Tcp => {
            println!("TCP IPv4");
            handle_tcp_packet(interface_name, source, destination, packet)
        }
        _ => {
            println!("Other");
        }
    }
}
