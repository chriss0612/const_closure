#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::items_after_statements)]
#![deny(rust_2018_idioms)]
#![deny(elided_lifetimes_in_paths)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![feature(const_refs_to_cell)]
#![feature(tuple_trait)]
#![doc = include_str!("../README.md")]

mod closure_type;
pub use closure_type::ConstClosure;

#[cfg(test)]
#[allow(clippy::trivially_copy_pass_by_ref)]
mod test;
