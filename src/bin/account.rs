extern crate meg;
extern crate log;

use log::*;

use std::env;
use std::clone::Clone;

use turbo::util::{CliResult, Config};
use self::meg::ops::meg_account_create as Act;
use self::meg::ops::meg_account_show as Show;




#[derive(RustcDecodable, Clone)]
pub struct Options {
pub flag_create: String,
pub flag_show: bool,
pub flag_verbose: bool,
}

pub const USAGE: &'static str = "
Usage:
    meg account [options]


Options:
    -h, --help               Print this message
    --create EMAIL           Provide an email to create a new account
    --show                   View your account details
    -v, --verbose            Use verbose output
";


pub fn execute(options: Options, config: &Config) -> CliResult<Option<()>> {
    debug!("executing; cmd=meg-account; args={:?}", env::args().collect::<Vec<_>>());

    config.shell().set_verbose(options.flag_verbose);

         let vec = env::args().collect::<Vec<_>>();
      for x in vec.iter() {
        if x == "--create" {
             let mut acct: Act::Createoptions  = Act::CreateAcc::new();
             acct.email = options.flag_create.clone();
             acct.create();

        } else if x == "--show" {
            let mut acct: Show::Showoptions  = Show::ShowAcc::new(); //Not reqd - to expand later if
            acct.email = options.flag_create.clone();                  //multiple accounts needs to be showed
            acct.show();

     }
 }

return Ok(None)

}
