use ethereum_types::{H160, H256};
use std::str::FromStr;

mod create2;

pub fn calc_create2_address_eth_types(
    sender: H160,
    salt: H256,
    code: Vec<u8>,
) -> Vec<u8> {
    create2::calc_create2_address(
        sender,
        salt,
        code,
    )
}

pub fn calc_create2_address_byte_vec(
    sender: Vec<u8>,
    salt: Vec<u8>,
    code: Vec<u8>,
) -> Vec<u8> {
    create2::calc_create2_address(
        H160::from_slice(sender.as_slice()),
        H256::from_slice(salt.as_slice()),
        code,
    )
}

pub fn calc_create2_address_string(
    sender: &str,
    salt: &str,
    code: &str,
) -> Vec<u8> {
    create2::calc_create2_address(
        H160::from_str(sender).expect("Invalid address string"),
        H256::from_str(salt).expect("Invalid salt string"),
        hex::decode(&code[2..]).expect("Invalid code"),
    )
}
