#![allow(unused)]

use hex;
use clap::{Args, Parser, Subcommand};
use rand::prelude::*;

#[derive(Parser)]
#[command(name = "create2calc")]
#[command(author = "Kalzak. <https://twitter.com/0xKalzak>")]
#[command(version = "1.0")]
#[command(about = "Calculates a create2 address or finds salt for a desired address", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Calc(CalcArgs),
    Find(FindArgs)
}

#[derive(Args)]
struct CalcArgs {
    #[arg(short, long)]
    address: String,
    #[arg(short, long)]
    salt: String,
    #[arg(short, long)]
    code: String,
}

#[derive(Args)]
struct FindArgs {
    #[arg(short, long)]
    address: String,
    #[arg(short, long)]
    code: String,
    #[arg(long, required_unless_present = "end")]
    start: Option<String>,
    #[arg(long, required_unless_present = "start")]
    end: Option<String>
}

fn main() {
    let mut cli = Cli::parse();

    match &cli.command {

        // Calculate one address
        Commands::Calc(calc_args) => {

            // Clone input so we can modify it
            let mut sender_str = calc_args.address.clone();
            let mut salt_str = calc_args.salt.clone();
            let mut code_str = calc_args.code.clone();

            // Remove the hex "0x" prepend if exists
            remove_hex_prepend(&mut sender_str);
            remove_hex_prepend(&mut salt_str);
            remove_hex_prepend(&mut code_str);
    
            // Pad sender address and salt str if necessary
            zero_pad_arg(&mut sender_str, 160/4);
            zero_pad_arg(&mut salt_str, 256/4);

            // Pad code to even bytes if necessary
            let code_len = code_str.len();
            if code_str.len() % 2 == 1 {
                zero_pad_arg(&mut code_str, code_len + 1); 
            }

            // Convert input to [u8; 32]
            let sender = hex::decode(sender_str).unwrap();
            let salt = hex::decode(salt_str).unwrap();
            let code = hex::decode(code_str).unwrap();

            let deployed = create2calc::calc_create2_address(sender, salt, code);

            println!("{}", hex::encode(deployed));

        },

        // Find salt for particular address
        Commands::Find(find_args) => {
            // Clone input so we can modify it
            let mut sender_str = find_args.address.clone();
            let mut code_str = find_args.code.clone();

            // Clone and process desired start and end address parts
            let (mut start, check_start) = process_address_part(find_args.start.clone());
            let (mut end, check_end) = process_address_part(find_args.end.clone());

            // Remove the hex "0x" prepend if exists
            remove_hex_prepend(&mut sender_str);
            remove_hex_prepend(&mut code_str);
            remove_hex_prepend(&mut start);
            remove_hex_prepend(&mut end);

            // Pad sender address if necessary
            zero_pad_arg(&mut sender_str, 160/4);

            // Pad code to even bytes if necessary
            let code_len = code_str.len();
            if code_str.len() % 2 == 1 {
                zero_pad_arg(&mut code_str, code_len + 1); 
            }

            // Convert input to [u8; 32]
            let sender = hex::decode(sender_str).unwrap();
            let code = hex::decode(code_str).unwrap();

            let mut rng = rand::thread_rng();
            let mut salt = [0_u8; 32];
            let mut deployed;

            loop {
                let mut found_start = false;
                let mut found_end = false;
                salt = rng.gen();

                deployed = create2calc::calc_create2_address(sender.clone(), salt, code.clone());
                let deployed_str = &hex::encode(deployed)[2..];

                // If looking for start match then check
                if check_start && (deployed_str[0..start.len()] == start) {
                    found_start = true;
                }

                // If looking for end match then check
                if check_end && (deployed_str[deployed_str.len() - end.len()..] == end) {
                    found_end = true;
                }
    
                // If both checks are met then output into to screen
                if !(check_start ^ found_start) && !(check_end ^ found_end) {
                        println!("Found: {}", deployed_str);
                        println!("For deployer: {}", hex::encode(sender.clone()));
                        println!("With init code: {}", hex::encode(code.clone()));
                        break;
                }
            }
        }
    }
}

fn process_address_part(address_part: Option<String>) -> (String, bool){
    if let Some(part) = address_part {
        (part, true)
    } else {
        ("".to_string(), false)
    }
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
