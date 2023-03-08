# Create2Calc

A program to calculate the address of a contract to be deployed using `create2` given a `sender` address, `salt` and `initcode`.
This is written in Rust and is my attempt to get more familiar with building Rust libraries.

Currently this program supports `String`, `Vec<u8>` or `H160/H256` (ethereum-types package) inputs.

Example input
```
Address = 0xf8e81D47203A594245E36C48e151709F0C19fBe8
Salt = 0x800e2ebd330b3c3a1b15462bc4b4f4f87c43f4e4ad30f76459c88ab9d3af3ce3
Code = 0x600b8060093d393df360026003015952596000f3

```

The list of things I want to improve
- [X] Enum arguments
- [ ] Returned address type matches `sender` argument type
- [ ] Add support for non-standard salt and address string lengths (eg: `0x1234`)
