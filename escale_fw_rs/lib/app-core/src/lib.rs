#![cfg_attr(not(test), no_std)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(let_chains)]

extern crate alloc;

pub mod button;
pub mod common;
pub mod input_scanner;
pub mod scale;
pub mod terminal;
