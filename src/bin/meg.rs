#![cfg_attr(unix, feature(fs_ext))]

extern crate turbo;
extern crate env_logger;
extern crate git2_curl;
extern crate rustc_serialize;
extern crate toml;
#[macro_use] extern crate log;

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io;
use std::path::{PathBuf, Path};
use std::process::Command;

use turbo::{execute_main_without_stdin, handle_error, shell};
use turbo::core::MultiShell;
use turbo::util::{CliError, CliResult, lev_distance, Config};

const USAGE: &'static str = "
Megam command line

Usage:
    meg <command> [<args>...]
    megam [options]

Options:
    -h, --help       Display this message
    -V, --version    Print version info and exit
    --list           List installed commands
    -v, --verbose    Use verbose output

Some common meg commands are:
    ahoy       Ping the status of megam.
    account    Create an account with megam.

See 'meg help <command>' for more information on a specific command.
";

fn main() {
    env_logger::init().unwrap();
    execute_main_without_stdin(execute, true, USAGE)
}

macro_rules! each_subcommand{ ($mac:ident) => ({
    $mac!(clean);
})}
