use ethereum_types::{H160, H256};
use hex;
use create2calc::{Sender, Salt, Code};


fn main() {
    /*
    let sender_string = Sender::Str("0xd9145CCE52D386f254917e481eB44e9943F39138".to_string());
    let salt_string = Salt::Str("0x800e2ebd330b3c3a1b15462bc4b4f4f87c43f4e4ad30f76459c88ab9d3af3ce3".to_string());
    let code_string = Code::Str("0x600e600c6000396000f300600036600073".to_string());

    let sender_ethtype = Sender::H160(H160::from_slice(&hex::decode(&"0xd9145CCE52D386f254917e481eB44e9943F39138"[2..]).unwrap()));
    let salt_ethtype = Salt::H256(H256::from_slice(&hex::decode(&"0x800e2ebd330b3c3a1b15462bc4b4f4f87c43f4e4ad30f76459c88ab9d3af3ce3"[2..]).unwrap()));

    let sender_vec = Sender::Bytes(hex::decode(&"0xd9145CCE52D386f254917e481eB44e9943F39138"[2..]).unwrap());
    let salt_vec = Salt::Bytes(hex::decode(&"0x800e2ebd330b3c3a1b15462bc4b4f4f87c43f4e4ad30f76459c88ab9d3af3ce3"[2..]).unwrap());
    let code_vec = Code::Bytes(hex::decode(&"0x600e600c6000396000f300600036600073"[2..]).unwrap());

    let mut deployed_address = create2calc::calc_create2_address(&sender_ethtype, &salt_ethtype, &code_vec);

    println!("0x{}", hex::encode(deployed_address));

    deployed_address = create2calc::calc_create2_address(&sender_vec, &salt_vec, &code_vec);

    println!("0x{}", hex::encode(deployed_address));

    deployed_address = create2calc::calc_create2_address(&sender_string, &salt_string, &code_string);

    println!("0x{}", hex::encode(deployed_address));
    */

    let sender = Sender::Str("0xf8e81D47203A594245E36C48e151709F0C19fBe8".to_string());
    let salt = Salt::Str("0x800e2ebd330b3c3a1b15462bc4b4f4f87c43f4e4ad30f76459c88ab9d3af3ce3".to_string());
    let code = Code::Str("0x600b8060093d393df360026003015952596000f3".to_string());

    let deployed_address = create2calc::calc_create2_address(&sender, &salt, &code);

    println!("0x{}", hex::encode(deployed_address));
}
