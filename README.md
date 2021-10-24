<img src="src/img/logo.png" height="120"/>

# termcrypt

![](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
<img src="http://ForTheBadge.com/images/badges/built-with-love.svg" height="26"/> ![](https://img.shields.io/github/commit-activity/m/termcrypt/termcrypt)

![](https://img.shields.io/badge/license-AGPL--3.0%2B-green)
![](https://img.shields.io/crates/v/termcrypt)
![](https://img.shields.io/badge/speed-like%20sonic-blue)

[<img src="src/img/xmr.gif" height="40"/>](https://termcrypt.github.io/donate/)

A terminal-like interface for efficiently trading the market.

Planned Exchanges termcrypt currently supports:

- [x] FTX
- [ ] Bybit

Right now the project is in very early development, so to test it you will most likely have to clone the source and will have to follow the steps on the build section below. 

We hope you enjoy termcrypt :)

[Our plans for features and fixes](https://github.com/termcrypt/termcrypt/projects).

### Features

termcrypt has many utilities that are growing day by day that help interact with the market. Some of the features include:

- Multiple exchange support (in future)
- Fast Speed (Only delays are API requests)
- Order with risk-ordersize calculations
- Subaccounts

For a more comprehensive list, run `help` or `h` inside of termcrypt.

<img src="src/img/example.png" height="80"/>

### Installation
Installing termcrypt is simple and it uses ~<500MB of space because it requires dependencies to compile.

1. Install rustup from [https://rustup.rs/](https://rustup.rs/) or by using another method if your platform is not supported.

2. Run the command: `cargo install termcrypt` in a terminal. We recommend [alacritty](https://alacritty.org/) for linux, mac and windows. We also recommend [termux](https://termux.com/) for android and [newterm](https://github.com/hbang/NewTerm) for ios (untested).

3. In the terminal, run `termcrypt`. You should see it asking for API keys, followed by ascii art. If you have any troubles, make an [issue](https://github.com/termcrypt/termcrypt/issues).

### Documentation

For more information on what termcrypt can do and commands, visit [termcrypt.github.io](https://termcrypt.github.io)

If you would like to contribute to documentation, visit [our docs repo](https://github.com/termcrypt/termcrypt.github.io).

### License

As developers, we believe that accountability leads to credibility and without the codebase being open, others cannot gain full trust. In addition, proprietary software does not allow external creativity and contribution which stunts unfunded smaller projects. We also believe that there should be no barrier to to market and it should be free for everybody no matter their current financial circumstances.

Because of this, we chose the AGPL-3.0-or-later license because it is one of the strongest copyleft licenses available.

**The APGL-3+ requires that termcrypt can not be used in any proprietary solution.**

### Contributing
If you want to open a pull request, you need to follow the instructions in CONTRIBUTING.md

### Build
#### Prerequisites
Following commands require `git`, `rustc` and `cargo`. 

`rustc` and `cargo` are installed by default with rustup.

Make sure you have the latest rustc version by running `rustup -V`.

#### Building

Clone the repo and open the directory:
```sh
git clone [repo url] termcrypt
cd termcrypt
```

Run / Build (for your os) with cargo:
```sh
cargo run
cargo build
```

Build files will be located in `target/debug`

### Hall of Fame
This section gives credit to people that have helped substantially in the development of this project.

- [fabianboesiger](https://github.com/fabianboesiger)
- conan ⚡️
- babdou