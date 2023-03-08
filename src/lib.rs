use ethereum_types::{H160, H256};

mod create2;

pub fn calc_create2_address<T: AsRef<[u8]>, U: AsRef<[u8]>, V: AsRef<[u8]>>(
    sender: T,
    salt: U,
    code: V,
) -> Vec<u8> {

    let sender_formatted = H160::from_slice(sender.as_ref());

    let salt_formatted = H256::from_slice(salt.as_ref());

    let code_formatted = code.as_ref().to_vec();

    create2::calc_create2_address(sender_formatted, salt_formatted, code_formatted)
}
