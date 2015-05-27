//#![deny(unused)]
//#![feature(fs_time)]
#![cfg_attr(test, deny(warnings))]

#[cfg(test)] extern crate hamcrest;
//#[macro_use] extern crate log;
extern crate docopt;

//use std::env;
//use std::error::Error;
//use std::io::prelude::*;
//use std::io;


//pub use util::{TurboError, CliError, CliResult, human, Config, ChainError};

//pub mod core;
pub mod ops;
//pub mod sources;
pub mod util;
