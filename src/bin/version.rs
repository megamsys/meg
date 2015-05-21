use std::env;

use turbo;
use turbo::util::{CliResult, Config};

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
    debug!("executing; cmd=meg-version; args={:?}", env::args().collect::<Vec<_>>());

    println!("{}", meg::version());

    Ok(None)
}
