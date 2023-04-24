[![Discord](https://img.shields.io/discord/1018936651612967043)](https://discord.gg/yMEKS2hk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub top language](https://img.shields.io/github/languages/top/hiro-codes/bolt)](https://github.com/hiro-codes/bolt/search?l=rust)

# Bolt ⚡
Bolt is a desktop application that is designed to make the process of developing and testing APIs easier and more efficient. Like Postman, but open source and written in Rust 🦀

Bolt can also be installed as a CLI app which allows you to run Bolt inside a web browser, with all the features of the Desktop app included.


![screenshot](https://github.com/hiro-codes/bolt/blob/master/screenshot.png?raw=true)

## Installation

> **Warning**: Bolt is experimental software. Expect:
> * Bugs
> * Missing features
> * Breaking changes

### Download pre-built binaries

Pre-built binaries for Windows, macOS and Linux can be found in the [latest release](https://github.com/hiro-codes/bolt/releases/latest) assets section.

### Build from source 👩‍💻

> ⚠️ Prerequisites
> 
> * [Rust](https://www.rust-lang.org/tools/install)
> * [Just](https://github.com/casey/just) or [Make](https://www.gnu.org/software/make/#download)


```bash
git clone https://github.com/hiro-codes/bolt
```

```bash
cd bolt
```

```bash
git checkout release
```

```bash
just setup # or make setup
```

```bash
just build # or make build
```


## Bolt CLI Installation

```bash
cargo install boltcli
```


### Usage

```bash
bolt
```

run `bolt -h` for command line options

### Build from source

> ⚠️ Prerequisites
> 
> * [Rust](https://www.rust-lang.org/tools/install)
> * [Just](https://github.com/casey/just) or [Make](https://www.gnu.org/software/make/#download)

```bash
git clone https://github.com/hiro-codes/bolt
```

```bash
cd bolt
```

```bash
git checkout release
```

```bash
just install-cli # or make install-cli
```


## Features 🚧
 * [x] Http Requests
 * [x] Collections
 * [x] CLI
 * [ ] Testing and benchmarking
 * [ ] Websockets
 * [ ] Logging
 * [ ] TCP/UDP

## Contributors ✨

<a href="https://github.com/hiro-codes/bolt/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=hiro-codes/bolt" />
</a>

## Donate/Sponsor ⭐
<a href="https://www.buymeacoffee.com/0xhiro" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-white.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;" ></a>

Made with ❤️  by [0xHiro](https://twitter.com/hiro_codes) 
