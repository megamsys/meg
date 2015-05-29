use std::env;
extern crate megam_api;

use turbo;
use turbo::util::{CliResult, Config};
use self::megam_api::util::sshkeys::SSHKey;
use self::megam_api::util::sshkeys::Success;




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

        let opts = SSHKey {
           name:   format!("{}", "Megam"),
           accounts_id: format!("{}", "Systems"),
           path: format!("{}", "00"),
        };
        let vec = env::args().collect::<Vec<_>>();
        for x in vec.iter() {
            if x == "--create" {

            let out = opts.create();
        match out {
        Ok(v) => {
            println!("success");
        }
        Err(e) => {
            println!("Error parsing header");
          }
        }} else if x == "--show" {
           }

   }

return Ok(None)

}
