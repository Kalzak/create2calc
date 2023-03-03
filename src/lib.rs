use ethereum_types::{H160, H256};
use std::str::FromStr;

mod create2;

pub enum Sender {
    Str(String),
    H160(H160),
    Bytes(Vec<u8>)
}

pub enum Salt {
    Str(String),
    H256(H256),
    Bytes(Vec<u8>)
}

pub enum Code {
    Str(String),
    Bytes(Vec<u8>)
}

pub fn calc_create2_address(
    sender: &Sender,
    salt: &Salt,
    code: &Code
) -> Vec<u8> {
    let sender_formatted: H160 = match sender {
        Sender::Str(s) => H160::from_str(s).expect("Invalid address string"),
        Sender::H160(s) => *s,
        Sender::Bytes(s) => H160::from_slice(s.as_slice())
    };

    let salt_formatted: H256 = match salt {
        Salt::Str(s) => H256::from_str(s).expect("Invalid salt string"),
        Salt::H256(s) => *s,
        Salt::Bytes(s) => H256::from_slice(s.as_slice())
    };

    let code_formatted: Vec<u8> = match code {
        Code::Str(c) => hex::decode(&c[2..]).expect("Invalid code"),
        Code::Bytes(c) => c.clone()
    };

    create2::calc_create2_address(sender_formatted, salt_formatted, code_formatted)
}
