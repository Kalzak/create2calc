use hex;

fn main() {
    
    let sender = hex::decode("f8e81D47203A594245E36C48e151709F0C19fBe8").unwrap();
    let salt = hex::decode("800e2ebd330b3c3a1b15462bc4b4f4f87c43f4e4ad30f76459c88ab9d3af3ce3").unwrap();
    let code = hex::decode("600b8060093d393df360026003015952596000f3").unwrap();

    let deployed_address = create2calc::calc_create2_address(sender, salt, code);

    println!("0x{}", hex::encode(deployed_address));
}
