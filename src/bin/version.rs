use std::env;

use turbo;
use turbo::util::{CliResult, Config};
extern crate term_painter;

use self::term_painter::ToStyle;
use self::term_painter::Color::*;


#[derive(RustcDecodable)]
struct Options;

pub const USAGE: &'static str = "
Usage:
    meg version [options]

Options:
    -h, --help              Print this message
    -v, --verbose           Use verbose output
";

pub fn execute(_: Options, _: &Config) -> CliResult<Option<()>> {
    //debug!("executing; cmd=meg-version; args={:?}", env::args().collect::<Vec<_>>());

    println!("{}",
     Green.bold().paint("\nMegam CLI - v0.1.0 - Launch torpedos(VMs), apps and services seamlessly on megam platform\n"));    Ok(None)
}
