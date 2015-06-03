extern crate term_painter;
extern crate meg;

use std::env;


use self::meg::ops::meg_sshkey_list as list;

use self::term_painter::ToStyle;
use self::term_painter::Color::*;
use self::term_painter::Attr::*;

use turbo;
use turbo::util::{CliResult, Config, TurboResult};


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
    println!("executing; cmd=meg-sshkey; args={:?}", env::args().collect::<Vec<_>>());



        let vec = env::args().collect::<Vec<_>>();
    for x in vec.iter() {
       if x == "--create" {

       } else if x == "--list" {
            println!("{}",
            Green.bold().paint("Listing SSHKey"));
            let x = list::SSHkeyoption;
            x.list();

        }
     }
 return Ok(None)
}
