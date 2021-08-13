#![deny(
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    while_true,
    clippy::panic,
    clippy::print_stdout,
    clippy::todo,
    //clippy::unwrap_used, // not yet in stable
    clippy::wrong_pub_self_convention
)]

#![doc = include_str!("../README.MD")]

/// Errors used in this crate
pub mod error;

/// Raw data types
/// 
/// These are used to then construct the main [`Aseprite`] type. 
pub mod raw;

/// Data structure representing an Aseprite file
pub struct Aseprite {}
