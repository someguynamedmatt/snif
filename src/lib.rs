extern crate pnet;
extern crate interfaces;

use std::net::IpAddr;
use self::interfaces::{Interface, Result};

use self::pnet::packet::Packet;
use self::pnet::packet::arp::ArpPacket;
use self::pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use self::pnet::packet::ipv4::Ipv4Packet;
use self::pnet::packet::tcp::TcpPacket;
use self::pnet::packet::ethernet::{EtherTypes, EthernetPacket};

/// Wrapper function to handle arbitrary data packets
pub fn handle_packet(interface_name: &str, ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(interface_name, ethernet),
        EtherTypes::Arp => handle_arp_packet(interface_name, ethernet),
        _ => println!("Other ethertype"),
    }
}

fn handle_arp_packet(interface_name: &str, ethernet: &EthernetPacket) {
    println!("Handle ARP packet!");
    let header = ArpPacket::new(ethernet.payload());
    if header.is_none() { println!("[{}]: Malformed ARP Packet", interface_name); return; }
    let header = header.unwrap();

    println!("[{}]: ARP packet: {}({}) > {}({}); operation: {:?}",
        interface_name,
        ethernet.get_source(),
        header.get_sender_proto_addr(),
        ethernet.get_destination(),
        header.get_target_proto_addr(),
        header.get_operation());
}

fn handle_ipv4_packet(interface_name: &str, ethernet: &EthernetPacket) {
    println!("Handle IPv4 Packet");
    let header = Ipv4Packet::new(ethernet.payload());
    if header.is_none() { println!("Malformed IPv4 Packet!!!"); return; }
    let header = header.unwrap();

    handle_transport_protocol(interface_name,
                              IpAddr::V4(header.get_source()),
                              IpAddr::V4(header.get_destination()),
                              header.get_next_level_protocol(),
                              header.payload());
}

fn handle_tcp_packet(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8]) {
    let tcp_packet = TcpPacket::new(packet);
    if tcp_packet.is_none() { println!("Malformed TCP packet"); return; }
    let tcp_packet = tcp_packet.unwrap();

    println!("[{}]: TCP Packet: {}:{} > {}:{}; length: {}",
             interface_name,
             source,
             tcp_packet.get_source(),
             destination,
             tcp_packet.get_destination(),
             packet.len());
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

pub fn change_interface_state(interface_name: &str, state_to: &str) {

    let on = "on";
    let off = "off";
    let new_state = match &state_to {
        &on => true,
        &off => false,
    };

    let mut i = match Interface::get_by_name(interface_name) {
        Ok(Some(i)) => i,
        Ok(None) => {
            println!("stuff");
            return;
        },
        Err(e) => {
            println!();
            return;
        },
    };

    /*
    match i.set_up(new_state) {
        Ok(_) => {
            println!("[OK]: Device is now whatever");
        },
        Err(e) => {
            // println!("[ERROR]: There was an error setting the device {}", new_state);
            println!("[ERROR]: setting device state");
        },
    };
    */
}

pub fn test_fn() {
    println!("HELLO FROM THE LIB.RS FILE!!!");
}
