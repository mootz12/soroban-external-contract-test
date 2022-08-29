#![no_std]

#[cfg(feature = "testutils")]
extern crate std;

use soroban_sdk::{contractimpl};

pub trait ExampleTrait {
    /// Some magic
    fn do_thing(x: u64) -> u64;
}

pub struct ExampleContract;

#[contractimpl]
impl ExampleTrait for ExampleContract {
    fn do_thing(x: u64) -> u64 {
        x + 1
    }
}

mod test;
