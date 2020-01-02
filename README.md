# libmygpo-rs

[![Build Status](https://travis-ci.org/FeuRenard/libmygpo-rs.svg?branch=master)](https://travis-ci.org/FeuRenard/libmygpo-rs)

Rust client library for [gpodder.net](https://gpodder.net/)

## Building
1. Clone this repository
2. Run `cargo build`

## Testing

1. Set environment variable `export GPODDER_NET_USERNAME=<username>`
2. Set environment variable `export GPODDER_NET_PASSWORD=<password>`
3. Set environment variable `export GPODDER_NET_DEVICEID=<deviceid>`
4. Run `cargo test -- --test-threads=1`

The tests have to run consecutively because they share state on the gpodder.net server.

## Install git hooks
* commit-msg: Run `ln -s ../../commit-msg.sh .git/hooks/commit-msg && chmod +x commit-msg.sh`
* pre-commit: Run `ln -s ../../pre-commit.sh .git/hooks/pre-commit && chmod +x pre-commit.sh`
