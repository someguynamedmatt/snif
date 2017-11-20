//! SNIF: Simple Network InterFace
//!
//! Quickly (and cleanly) check the configurations of your network devices

// Enable clippy if our Cargo.toml file asked us to do so.
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

// Enable as many useful Rust and Clippy warnings as we can stand.  We'd
// also enable `trivial_casts`, but we're waiting for
// https://github.com/rust-lang/rust/issues/23416.
#![warn(missing_copy_implementations,
        missing_debug_implementations,
        missing_docs,
        trivial_numeric_casts,
        unsafe_code,
        unused_extern_crates,
        unused_import_braces,
        unused_qualifications)]
#![cfg_attr(feature="clippy", warn(cast_possible_truncation))]
#![cfg_attr(feature="clippy", warn(cast_possible_wrap))]
#![cfg_attr(feature="clippy", warn(cast_precision_loss))]
#![cfg_attr(feature="clippy", warn(cast_sign_loss))]
#![cfg_attr(feature="clippy", warn(missing_docs_in_private_items))]
#![cfg_attr(feature="clippy", warn(mut_mut))]
// Disallow `println!`. Use `debug!` for debug output
// (which is provided by the `log` crate).
#![cfg_attr(feature="clippy", warn(print_stdout))]
// This allows us to use `unwrap` on `Option` values (because doing makes
// working with Regex matches much nicer) and when compiling in test mode
// (because using it in tests is idiomatic).
#![cfg_attr(all(not(test), feature="clippy"), warn(result_unwrap_used))]
#![cfg_attr(feature="clippy", warn(unseparated_literal_suffix))]
#![cfg_attr(feature="clippy", warn(wrong_pub_self_convention))]

extern crate pnet_datalink;
extern crate ipnetwork;
extern crate pnet;

use std::env;
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
    let iface_arg = env::args().nth(1).expect("Network interface name not supplied!");

    let interface_match = |iface: &pnet_datalink::NetworkInterface| iface.name == iface_arg;
    let interface = interfaces.into_iter().find(interface_match).unwrap();

    println!("===================");
    println!("IP:");
    for ip in &interface.ips {
        match *ip {
            IpNetwork::V4(a) => println!("   IPv4: {}", a),
            IpNetwork::V6(a) => println!("   IPv6: {}", a),
        }
    }

    println!("-------------------");
    println!("Mac Addr:");
    println!("   {}", interface.mac.expect("???"));
    println!("===================");

    let (_, mut rx) = match pnet_datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type!"),
        Err(e) => panic!("An error occurred when creating the datalink: {}", e)
    };

    loop {
        let packet = EthernetPacket::new(rx.next().expect("ERROR WITH PACKET!!!"));
        handle_packet("wlp4s0", &packet.unwrap());
    }
}

/// Wrapper function to handle arbitrary data packets
fn handle_packet(interface_name: &str, ethernet: &EthernetPacket) {
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

#[cfg(test)]
mod test {
    #[test]
    fn easy_test() {
        assert!(1 == 1)
    }
}
