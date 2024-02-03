# icredeem

[![Build Status](https://github.com/zarthus/idle_champions_redeemer/actions/workflows/rust.yml/badge.svg)](https://github.com/zarthus/idle_champions_redeemer/actions)
[![Docs.rs](https://docs.rs/icredeem/badge.svg)](https://docs.rs/icredeem/latest/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](README#license)

CLI tool to redeem Idle Champions of the Forgotten Realms codes.

Set up cursor positions in the config file, and run the tool with a list of codes to redeem.

This can interface with [idle_champions_codes_api](https://github.com/Liefland/idle_champions_codes_api) hosted repositories, of which
the official one maintained by Liefland is hosted at [codes.idlechampions.liefland.net](https://codes.idlechampions.liefland.net/)

All repositories we maintain: [GitHub](https://github.com/Liefland?q=idle_champions)

## Installation

Install as software:
- `cargo install icredeem`

Without connecting to the remote API (no `tokio` or `reqwest` dependencies):
- `cargo install icredeem --no-default-features`

You can the run it like so: 
- `icredeem setup` to generate a config file.
- `icredeem` use the default strategy based on the config, this will defalut to getting redeemable codes from the API
- `icredeem --codes="NEWA-CCOU-NTNE-WME!"` to redeem a code.
- `icredeem --prefer-remote` if the configured strategy is local, this will use the remote strategy for this call.

## Uninstalling

- (optional) Remove the config file, you can use `icredeem clean` in the binary to do this for you.
- Remove the binary from the system

## Recommended reading

- If you have no intention of using the remote API to redeem codes, 
  we recommend installing with `--no-default-features` to avoid the `tokio` and `reqwest` dependencies.
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
