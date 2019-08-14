extern crate interfaces;

use self::interfaces::{Interface};
use std::io;
use std::io::prelude::*;

pub fn change_interface_state(interface_name: &str, state_to: &str) {
    println!("STATE_TO {}", &state_to);

    let mut new_state: bool = true;
    if state_to == "off" {
        new_state = false;
    }

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

    match i.set_up(new_state) {
        Ok(_) => {
            if new_state == true {
                io::stdout().write("[OK]: Device is now on\n".as_bytes());
            }
            if new_state == false {
                io::stdout().write("[OK]: Device is now off\n".as_bytes());
            }
        },
        Err(e) => {
            println!("[ERROR]: There was an error setting the device {}", e);
        },
    };
}
