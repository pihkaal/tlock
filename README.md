<h1 align="center">
    <br>
    <img src="https://i.imgur.com/EeTzHDR.png" width="200">
    <br>
    tlock
    <br>
</h1>

<h4 align="center">Fully customizable terminal clock.</h4>

<p align="center">
    <a href="https://www.rust-lang.org">
        <img src="https://img.shields.io/badge/rust-f54b00?style=for-the-badge&logo=rust&logoColor=white">
    </a>
</p>

<p align="center" id="links">
    <a href="#description">Description</a> •
    <a href="#installation">Installation</a> •
    <a href="#how-to-use">How to use</a> •
    <a href="#configuration">Configuration</a> •
    <a href="https://pihkaal.me">Visit it</a> •
    <a href="#license">License</a>
</p>

<br>

## Description

This is a fully customizable terminal clock written in Rust. You can change de colors, the format and you can even use multiples modes: clock, chronometer and timer.

<br>

## Installation

```bash
$ cargo install --git https://github.com/pihkaal/tlock.git
```

<br>

## How to use

```bash
# Help
$ tlock --help

# Clock mode
$tlock

# Debug mode (print current configuration)
$ tlock debug

# Chronometer mode
$ tlock chrono

# Timer mode
$ tlock timer 4h 12m 30s
```

<br>

## Configuration

Configuration is stored under `~/.config/tlock/config`, it is generated by the program if it doesn't exist.  
You can regenerate this configuration at any time by running:

```bash
$ tlock --regenerate-default
```

You can use multiple configuration files thanks to the `--config` flag:

```bash
$ tlock --config /path/to/my/config
```

The configuration itself contains comments to help you understand how to customize it.

<br>

## License

This project is <a href="https://opensource.org/licenses/MIT">MIT</a> licensed.
