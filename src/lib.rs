//! This is a client library for the [gpodder.net](https://gpodder.net/) API
//!
//! # Usage
//!
//! For any request you need a [client](./client/index.html#structs).
//!
//! All supported API features are represented by a trait.
//!
//! A client's implemented traits mark its capabilities.

#![deny(
    clippy::all,
    warnings,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rustdoc,
    unused,
    macro_use_extern_crate,
    missing_docs,
    non_ascii_idents,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications
)]
#![allow(missing_doc_code_examples)]

pub mod client;
pub mod device;
pub mod directory;
pub mod episode;
pub mod error;
pub mod settings;
pub mod subscription;
pub mod suggestion;
