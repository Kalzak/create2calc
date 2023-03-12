# Create2Calc

A program to calculate the address deployed from a contract using `create2`. It can also be used to find a salt which generates an address with a particular sequence of start or end hex values. This is written in Rust to encourage myself to become more familiar with the language and to learn its unique features. 

## How to use

The program can be used as follows:

1. Calculate the deployed address given a deployed address, salt and initcode:
   ```
   ./create2calc calc <address> <salt> <code>
   ```
   Example:
   ```
   ./create2calc calc --address 0x1234 --code 0x1234567890 --salt 0x4321
   Deployed address: 1892225c1b09454c18418c380603021c26e8b6ff
   ```

2. Find some salt to deploy an address with the specific sequence of hex values:
   ```
   ./create2calc find <address> <salt> --start <sequence> --end <sequence>
   ```
   Example:
   ```
   ./create2calc find --address 0x1234 --code 0x1234567890 --start 0x222 --end 0x333
   Found: 2222cbd6bc152fab9b5312a333de108e63c333
   For deployer: 0000000000000000000000000000000000001234
   With init code: 1234567890
   Using salt: f9327c7c264fb331cf8ad0f22c2759770ebe835b469e4f3382e654b02c4b024b
   
   ./create2calc find --address 0x1 --code 0x111 --start 0x1234
   Found: 123407b6d1c03cd97331e8b4e01e529b278e6b
   For deployer: 0000000000000000000000000000000000000001
   With init code: 0111
   Using salt: 06e03da6a716735d984ee55f0d799457c9ea1cc01049e285fd01cb7427d4ddc0
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
- [X] Add support for bruteforcing desired beginning or end hex sequences
  - [ ] Multithreading
