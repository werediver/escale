#![cfg_attr(not(test), no_std)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

extern crate alloc;

pub mod mq;
pub mod ring;
mod ring_state;
pub mod run_loop;
pub mod signal;
