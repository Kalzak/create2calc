# Create2Calc

A program to calculate the address deployed from a contract using `create2`. This is written in Rust to encourage myself to become more familiar with the language and to learn its unique features. 

## How to use

The program can be used as follows:

```
./create2calc <address> <salt> <code>
```

The arguments work with or without the hex `0x` prepend.

## Improvements

The following is a list of previously completed or future improvements:

- [X] Enum arguments
- [X] Generic arguments
- [X] Add support for non-standard arguments
  - [X] Varying lengths for `address` and `salt`
  - [X] Hex input `0x` prepend optional
- [X] Add CLI interface rather than hardcoding values
- [X] Move important logic into separate library
- [ ] Add support for bruteforcing desired beginning or end hex sequences
  - [ ] Multithreading
