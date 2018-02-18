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
                      .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    //println!("Using version: {}", matches.value_of("v").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.value_of("d").unwrap() {
        "stuff" => println!("STUFFFFF"),
        _ => println!("Don't be crazy"),
    }

    /*
    let interfaces = pnet_datalink::interfaces();
    let iface_arg = env::args().nth(1).expect("Network interface name not supplied!");

    let interface_match = |iface: &pnet_datalink::NetworkInterface| iface.name == iface_arg;
    let interface = interfaces.into_iter().find(interface_match).unwrap();
    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }


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
