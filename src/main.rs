extern crate pnet_datalink;
extern crate ipnetwork;

use std::env;
use std::io::{self, Write};
use std::process;
use ipnetwork::IpNetwork;

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

    println!("IPs:");
    for ip in interface.ips {
        println!("    {}", IpNetwork::ip(&ip).to_string());
    }
}
