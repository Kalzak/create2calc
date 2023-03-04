use ethereum_types::{H160, H256};
use sha3::{Digest, Keccak256};

pub(crate) fn calc_create2_address(
    sender: H160,
    salt: H256,
    code: Vec<u8>,
) -> Vec<u8> {
    let codehash = hash_code(code);
    hash_create2_args(sender, salt, codehash)[12..].to_vec()
}

fn hash_create2_args(
    sender: H160,
    salt: H256,
    initcodehash: Vec<u8>,
) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    let constant = vec![255u8]; // 0xff
    
    hasher.update(constant);
    hasher.update(sender);
    hasher.update(salt);
    hasher.update(initcodehash);

    hasher.finalize().to_vec()
}

fn hash_code(initcode: Vec<u8>) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(initcode);
    hasher.finalize().to_vec()
}
