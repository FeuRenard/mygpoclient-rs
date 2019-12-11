# libmygpo-rs
Rust client library for [gpodder.net](https://gpodder.net/)

## Building
1. Clone this repository
2. Run `cargo build`

## Testing
1. Set environment variable `export GPODDER_NET_USERNAME="<username>"`
2. Set environment variable `export GPODDER_NET_PASSWORD="<password>"`
3. Run `cargo test`

## Install git hooks
* commit-msg: Run `ln -s ../../commit-msg.sh .git/hooks/commit-msg && chmod +x commit-msg.sh`
* pre-commit: Run `ln -s ../../pre-commit.sh .git/hooks/pre-commit && chmod +x pre-commit.sh`
