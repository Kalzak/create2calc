# Create2Calc

A program to calculate the address of a contract to be deployed using `create2` given a `sender` address, `salt` and `initcode`.
This is written in Rust and is my attempt to get more familiar with building Rust libraries.

Currently this program supports `String`, `Vec<u8>` or `H160/H256` (ethereum-types package) inputs.