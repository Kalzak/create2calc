use ethereum_types::{H160, H256};
use sha3::{Digest, Keccak256};
use hex;

pub fn calc_create2_address(
    sender_string: &str,
    salt_string: &str,
    initcode_string: &str,
) -> String {

    let constant = vec![255u8]; // 0xff

    let sender = H160::from_slice(&hex::decode(&sender_string[2..]).unwrap());
    let salt = H256::from_slice(&hex::decode(&salt_string[2..]).unwrap());
    let initcode = hex::decode(&initcode_string[2..]).unwrap();

    let initcodehash = hash_initcode(initcode);

    let hashoutput = hash_create2_args(constant, sender, salt, initcodehash);

    format!("0x{}", hex::encode(hashoutput[12..].to_vec()))
}

fn hash_initcode(initcode: Vec<u8>) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(initcode);
    hasher.finalize().to_vec()
}

fn hash_create2_args(
    constant: Vec<u8>,
    sender: H160,
    salt: H256,
    initcodehash: Vec<u8>,
) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    
    hasher.update(constant);
    hasher.update(sender);
    hasher.update(salt);
    hasher.update(initcodehash);

    hasher.finalize().to_vec()
}
