# Idle Champions Redeemer

[![Build Status](https://github.com/zarthus/idle_champions_redeemer/actions/workflows/rust.yml/badge.svg)](https://github.com/zarthus/idle_champions_redeemer/actions)
[![Docs.rs](https://docs.rs/idle_champions_redeemer/badge.svg)](https://docs.rs/idle_champions_redeemer/latest/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](README#license)

CLI tool to redeem Idle Champions of the Forgotten Realms codes.

Set up cursor positions in the config file, and run the tool with a list of codes to redeem.

## Installation

Install as software:
- `cargo install icredeem`

You can the run it like so: 
- `icredeem setup` to generate a config file.
- `icredeem --codes="NEWA-CCOU-NTNE-WME!"` to see the available options.

## Uninstalling

- (optional) Remove the config file, you can use `icredeem clean` in the binary to do this for you.
- Remove the binary from the system

## Recommended reading

- We use arboard to manipulate your clipboard.
  - For those who use wayland, the optional `wayland` feature and https://github.com/1Password/arboard?tab=readme-ov-file#gnulinux may be interesting to you. 
- We use Enigo to simulate mouse clicks and keyboard presses, it has some dependency and permission notes:
  - https://github.com/enigo-rs/enigo/blob/main/Permissions.md
  - https://github.com/enigo-rs/enigo#runtime-dependencies

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

Licensed under the following licenses at your option:

- Apache License, Version 2.0 <[LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0>
- MIT license <[LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT>

Files in the project may not be copied, modified, or distributed except according to those terms.
