extern crate term_painter;
extern crate meg;

use std::env;


use self::meg::ops::meg_sshkey_list as list;

use turbo::util::{CliResult, Config};


#[derive(RustcDecodable)]
struct Options;

pub const USAGE: &'static str = "
Usage:
    meg sshkey [options]


Options:
    -h, --help              Print this message
    --list                  List SSHKeys
";


pub fn execute(_: Options, _: &Config) -> CliResult<Option<()>> {



    let vec = env::args().collect::<Vec<_>>();
    for x in vec.iter() {
       if x == "--create" {
       } else if x == "--list" {
            let x = list::SSHkeyoption;
            x.list();
        }
     }
 return Ok(None)
}
