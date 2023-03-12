#![allow(unused)]

use hex;
use clap::Parser;
use rand::prelude::*;

#[derive(Parser)]
#[command(name = "create2calc")]
#[command(author = "Kalzak. <https://twitter.com/0xKalzak>")]
#[command(version = "1.0")]
#[command(about = "Calculates a create2 address or finds salt for a desired address", long_about = None)]

struct Cli {
    #[arg(short, long)]
    address: String,
    #[arg(short, long)]
    salt: String,
    #[arg(short, long)]
    code: String,
    #[arg(short, long, default_value_t = false)]
    find: bool,
    #[arg(long)]
    start: Option<String>,
    #[arg(long)]
    end: Option<String>
}

fn main() {
    let mut args = Cli::parse();

    process_args(&mut args);
    
    let sender = hex::decode(args.address).unwrap();
    let code = hex::decode(args.code).unwrap();

    // If only calculating one address
    if !args.find {
        let salt = hex::decode(args.salt).unwrap();
        let deployed_address = create2calc::calc_create2_address(sender, salt, code);
        println!("0x{}", hex::encode(deployed_address));

    // If finding a salt for a particular address
    } else {
        let mut rng = rand::thread_rng();
        let mut salt = [0_u8; 32];
        let mut deployed_address;

        let start;
        let check_start;

        if let Some(mut start_arg) = args.start {
            remove_hex_prepend(&mut start_arg);
            start = start_arg;
            check_start = true;
        } else {
            start = "".to_string();
            check_start = false;
        }

        let end;
        let check_end;

        if let Some(mut end_arg) = args.end {
            remove_hex_prepend(&mut end_arg);
            end = end_arg;
            check_end = true;
        } else {
            end = "".to_string();
            check_end = false;
        }

        loop {
            let mut start_found = false;
            let mut end_found = false;
            salt = rng.gen();
            deployed_address = create2calc::calc_create2_address(sender.clone(), salt, code.clone());
            let deployed_address_string = &hex::encode(deployed_address)[2..];
            if check_start {
                if deployed_address_string[0..(start.len())] == start {
                    start_found = true;
                }
            }
            if check_end {
                if deployed_address_string[(deployed_address_string.len() - end.len())..] == end {
                    end_found = true;
                }
            }

            if !(check_start ^ start_found) && !(check_end ^ end_found) {
                    println!("{}", deployed_address_string);
            }
        }
    }
}

fn process_args(args: &mut Cli) {
    // Remove the hex "0x" prepend if exists
    remove_hex_prepend(&mut args.address);
    remove_hex_prepend(&mut args.salt);
    remove_hex_prepend(&mut args.code);
    
    // Pad sender address and salt string if necessary
    zero_pad_arg(&mut args.address, 160/4);
    zero_pad_arg(&mut args.salt, 256/4);
}

fn remove_hex_prepend(arg: &mut String) {
    if arg.starts_with("0x") {
        arg.remove(0);
        arg.remove(0);
    }
}

fn zero_pad_arg(arg: &mut String, desired_length: usize) {
    let arg_length = arg.len();
    if arg_length < desired_length {
        let zeroes = "0".repeat(desired_length - arg_length);
        arg.insert_str(0, &zeroes);
    }
}
