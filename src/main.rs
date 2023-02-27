mod create2;

fn main() {
    let sender_string = "0xd9145CCE52D386f254917e481eB44e9943F39138";
    let salt_string = "0x800e2ebd330b3c3a1b15462bc4b4f4f87c43f4e4ad30f76459c88ab9d3af3ce3";
    let initcode_string = "0x600e600c6000396000f300600036600073";

    let deployed_address = create2::calc_create2_address(sender_string, salt_string, initcode_string);

    println!("{}", deployed_address);
}
