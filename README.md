# mygpoclient-rs

[![Build Status](https://travis-ci.org/FeuRenard/mygpoclient-rs.svg?branch=master)](https://travis-ci.org/FeuRenard/mygpoclient-rs)

Rust client library for [gpodder.net](https://gpodder.net/)

## Building
1. Clone this repository
2. Run `cargo build`

## Testing

1. Run `cp set-credentials.sh _set-credentials.sh`
2. Enter valid gpodder.net credentials in `_set-credentials.sh`
3. Run `source ./_set-credentials.sh`
4. Run `cargo test -- --test-threads=1`

The tests have to run consecutively because they share state on the gpodder.net server.

## Install git hooks
* commit-msg: Run `ln -s ../../commit-msg.sh .git/hooks/commit-msg && chmod +x commit-msg.sh`
* pre-commit: Run `ln -s ../../pre-commit.sh .git/hooks/pre-commit && chmod +x pre-commit.sh`
