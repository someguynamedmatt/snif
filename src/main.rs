//! `SNIf`: Simple Network Interface
//!
//! Quickly (and cleanly) check the configurations of your network devices

extern crate clap;
extern crate ipnetwork;
extern crate pnet_datalink;
extern crate pnet;

use std::env;
use std::io;
use std::io::prelude::*;
use clap::{Arg, App, SubCommand};
use ipnetwork::IpNetwork;
use pnet_datalink::Channel::Ethernet;
use pnet::packet::ethernet::EthernetPacket;

mod lib;
use lib::*;

#[cfg(not(test))]
fn main() {
    let matches = App::new("SNIf")
                      .version("1.0")
                      .author("Matt <young.qubit@gmail.com>")
                      .about("Simple Network Interface tool")
                      .arg(Arg::with_name("d")
                           .short("d")
                           .long("device")
                           .value_name("STRING")
                           .help("device name"))
                      .arg(Arg::with_name("s")
                           .short("s")
                           .long("state")
                           .value_name("STRING")
                           .help("device state (on/off)\n\n"))
                      .get_matches();

    if env::args().len() == 1 {
        io::stdout().write("\n======== Devices ========\n".as_bytes());
        for interface in pnet_datalink::interfaces() {
            println!("* {}", interface.name);
        }
        io::stdout().write("=========================\n".as_bytes());
        io::stdout().write("For more detailed output try: `snif -d <device name>`\n\n".as_bytes());
    }
    let interfaces = pnet_datalink::interfaces();

    if let Some(device) = matches.value_of("d") {
        let interface_match = |iface: &pnet_datalink::NetworkInterface| iface.name == device;
        let interface = interfaces.into_iter().find(interface_match).unwrap();

        io::stdout().write("\n=========================\n".as_bytes());
        io::stdout().write("IP:\n".as_bytes());
        for ip in &interface.ips {
            match *ip {
                IpNetwork::V4(a) => println!("   IPv4: {}", a),
                IpNetwork::V6(a) => println!("   IPv6: {}", a),
            }
        }

        io::stdout().write("-------------------------\n".as_bytes());
        io::stdout().write("Mac Addr:\n".as_bytes());
        let mac_address = interface.mac.expect("???");
        io::stdout().write(mac_address.to_string().as_bytes());
        io::stdout().write("\n".as_bytes());
        io::stdout().write("=========================\n\n".as_bytes());

        if let Some(device_state) = matches.value_of("s") {
            change_interface_state(&device, &device_state);
        }
    }

    /*
    let iface_arg = env::args().nth(1).expect("Network interface name not supplied!");

    let interface_match = |iface: &pnet_datalink::NetworkInterface| iface.name == device;
    let interface = interfaces.into_iter().find(interface_match).unwrap();
    */

    /*
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

    let requested_device_state = env::args().nth(2).expect("Device state not defined!");

    change_interface_state(&iface_arg, &requested_device_state);

    let (_, mut rx) = match pnet_datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type!"),
        Err(e) => panic!("An error occurred when creating the datalink (maybe run as sudo):\n\n {}", e)
    };

    loop {
        let packet = EthernetPacket::new(rx.next().expect("ERROR WITH PACKET!!!"));
        lib::handle_packet("wlp4s0", &packet.unwrap());
    }
    */
}
