extern crate meg;

use std::env;
use std::clone::Clone;

use turbo::util::{CliResult, Config};
use self::meg::ops::meg_account_create as Act;
use self::meg::ops::meg_account_show as Show;




#[derive(RustcDecodable, Clone)]
pub struct Options {
pub arg_email: String,
pub flag_show: bool,
}

pub const USAGE: &'static str = "
Usage:
    meg account [options] [<email>]


Options:
    -h, --help               Print this message
    --create                 Provide an email to create a new account
    --show                   Provide an email to show the account
    -v, --verbose            Use verbose output
";


pub fn execute(options: Options, _: &Config) -> CliResult<Option<()>> {

         let vec = env::args().collect::<Vec<_>>();
      for x in vec.iter() {
        if x == "--create" {
             let mut acct: Act::Createoptions  = Act::CreateAcc::new();
             acct.email = options.arg_email.clone();
             let x = acct.create();

        } else if x == "--show" {
            let mut acct: Show::Showoptions  = Show::ShowAcc::new(); //Not reqd - to expand later if
            acct.email = options.arg_email.clone();                  //multiple accounts needs to be showed
            let x = acct.show();

     }
 }

return Ok(None)

}
