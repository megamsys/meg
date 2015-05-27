
use std::env;
use std::io;
use std::io::prelude::*;

use turbo;
use turbo::util::{CliResult, Config};
use meg::ops;



#[derive(RustcDecodable)]
pub struct Options {
arg_email: Option<String>,
}

pub const USAGE: &'static str = "
Usage:
    meg account [options] [<email>]

Options:
    -h, --help               Print this message
    --create           Provide an email to create a new account
    --show              Provide an email to show the account
    -v, --verbose            Use verbose output
";


pub fn execute(options: Options, _: &Config) -> CliResult<Option<()>> {
    println!("executing; cmd=meg-version; args={:?}", env::args().collect::<Vec<_>>());
    println!("efefefefe{:?}", options.arg_email);
    let det = ops::AccountCreateFields {
            email: options.arg_email,
    };
    //let stdin =  io::stdin();
    //let readBytes = stdin.lock().lines();
    //println!("{:?}", readBytes);
return Ok(None)

}

/*
pub fn Send() {
    let det = ops::AccountCreateFields {
            email: options.arg_email,
    };

}
*/
