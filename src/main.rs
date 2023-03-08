#![allow(unused)]

use hex;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    sender: String,
    salt: String,
    code: String
}

fn main() {
    let mut args = Cli::parse();

    process_args(&mut args);
    
    let sender = hex::decode(args.sender).unwrap();
    let salt = hex::decode(args.salt).unwrap();
    let code = hex::decode(args.code).unwrap();

    let deployed_address = create2calc::calc_create2_address(sender, salt, code);

    println!("0x{}", hex::encode(deployed_address));
}

fn process_args(args: &mut Cli) {
    // Remove the hex "0x" prepend if exists
    remove_hex_prepend(&mut args.sender);
    remove_hex_prepend(&mut args.salt);
    remove_hex_prepend(&mut args.code);

    // Pad sender and salt string if necessary
    zero_pad_arg(&mut args.sender, 160/4);
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
