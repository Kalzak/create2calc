use ethereum_types::{H160, H256};
use hex;

fn main() {
    let sender_string = "0xd9145CCE52D386f254917e481eB44e9943F39138";
    let salt_string = "0x800e2ebd330b3c3a1b15462bc4b4f4f87c43f4e4ad30f76459c88ab9d3af3ce3";
    let initcode_string = "0x600e600c6000396000f300600036600073";

    let sender_ethtype = H160::from_slice(&hex::decode(&sender_string[2..]).unwrap());
    let salt_ethtype = H256::from_slice(&hex::decode(&salt_string[2..]).unwrap());

    let sender_vec = hex::decode(&sender_string[2..]).unwrap();
    let salt_vec = hex::decode(&salt_string[2..]).unwrap();
    let initcode_vec = hex::decode(&initcode_string[2..]).unwrap();

    let mut deployed_address = create2calc::calc_create2_address_eth_types(sender_ethtype, salt_ethtype, initcode_vec.clone());

    println!("0x{}", hex::encode(deployed_address));

    deployed_address = create2calc::calc_create2_address_byte_vec(sender_vec, salt_vec, initcode_vec);

    println!("0x{}", hex::encode(deployed_address));

    deployed_address = create2calc::calc_create2_address_string(sender_string, salt_string, initcode_string);

    println!("0x{}", hex::encode(deployed_address));

}
