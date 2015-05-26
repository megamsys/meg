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
    //debug!("executing; cmd=meg-version; args={:?}", env::args().collect::<Vec<_>>());

    println!("{}", self::version());

    Ok(None)
}

pub fn version() -> String {

    //println!("Commencing yak shaving for 0$ {}", option_env!("MEG_PKG_VERSION_MAJOR"));
println!("Inside version-");
    format!("meg {}", match option_env!("CFG_VERSION") {
        Some(s) =>  s.to_string(),
        None    =>   format!("{}.{}.{}{}",
                        env!("CARGO_PKG_VERSION_MAJOR"),
                        env!("CARGO_PKG_VERSION_MINOR"),
                        env!("CARGO_PKG_VERSION_PATCH"),
                        env!("CARGO_PKG_VERSION_PRE"))
    })
}
