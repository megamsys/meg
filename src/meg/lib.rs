#![deny(unused)]
#![feature(fs_time)]
#![cfg_attr(test, deny(warnings))]

#[cfg(test)] extern crate hamcrest;
#[macro_use] extern crate log;
extern crate docopt;

use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::io;


pub use util::{TurboError, CliError, CliResult, human, Config, ChainError};

pub mod core;
pub mod ops;
pub mod sources;
pub mod util;



pub fn version() -> String {

    println!("Commencing yak shaving for 0$ {}", env!("MEG_PKG_VERSION_MAJOR"));

    format!("meg {}", match option_env!("CFG_VERSION") {
        Some(s) =>  s.to_string(),
        None    =>   format!("{}.{}.{}{}",
                        env!("MEG_PKG_VERSION_MAJOR"),
                        env!("MEG_PKG_VERSION_MINOR"),
                        env!("MEG_PKG_VERSION_PATCH"),
                        option_env!("MEG_PKG_VERSION_PRE").unwrap_or(""))
    })
}
