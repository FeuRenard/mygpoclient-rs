# mygpoclient-rs


[![Build Status](https://travis-ci.org/FeuRenard/mygpoclient-rs.svg?branch=master)](https://travis-ci.org/FeuRenard/mygpoclient-rs)
[![codecov](https://codecov.io/gh/FeuRenard/mygpoclient-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/FeuRenard/mygpoclient-rs)
[![coveralls](https://coveralls.io/repos/github/FeuRenard/mygpoclient-rs/badge.svg?branch=master)](https://coveralls.io/github/FeuRenard/mygpoclient-rs?branch=master)
[![libraries.io](https://img.shields.io/librariesio/github/FeuRenard/mygpoclient-rs)](https://libraries.io/github/FeuRenard/mygpoclient-rs)
[![Gitmoji](https://img.shields.io/badge/gitmoji-%20😜%20😍-FFDD67.svg?style=flat)](https://gitmoji.carloscuesta.me)
[![Crates.io](https://img.shields.io/crates/v/mygpoclient)](https://crates.io/crates/mygpoclient)
[![Documentation](https://docs.rs/mygpoclient/badge.svg)](https://docs.rs/mygpoclient)

Rust client library for [gpodder.net](https://gpodder.net/)

## Supported features

- [ ] [Authentication](https://gpoddernet.readthedocs.io/en/latest/api/reference/auth.html)
- [x] [Directory](https://gpoddernet.readthedocs.io/en/latest/api/reference/directory.html)
- [x] [Suggestions](https://gpoddernet.readthedocs.io/en/latest/api/reference/suggestions.html)
- [x] [Device](https://gpoddernet.readthedocs.io/en/latest/api/reference/devices.html)
- [x] [Subscriptions](https://gpoddernet.readthedocs.io/en/latest/api/reference/subscriptions.html)
- [x] [Episode Actions](https://gpoddernet.readthedocs.io/en/latest/api/reference/events.html)
- [ ] [Podcast Lists](https://gpoddernet.readthedocs.io/en/latest/api/reference/podcastlists.html)
- [x] [Settings](https://gpoddernet.readthedocs.io/en/latest/api/reference/settings.html)
- [x] [Favorites](https://gpoddernet.readthedocs.io/en/latest/api/reference/favorites.html)
- [ ] [Device Synchronization](https://gpoddernet.readthedocs.io/en/latest/api/reference/sync.html)
- [ ] [Client Parametrization](https://gpoddernet.readthedocs.io/en/latest/api/reference/clientconfig.html)

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
