use std::env;

use turbo;
use turbo::util::{CliResult, Config};



#[derive(RustcDecodable)]
struct Options;

pub const USAGE: &'static str = "
Usage:
    meg sshkey [options]

Options:
    -h, --help              Print this message
";


pub fn execute(_: Options, _: &Config) -> CliResult<Option<()>> {

println!("Inside sshkeys-------------->>");
return Ok(None)

}
