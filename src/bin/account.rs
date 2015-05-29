extern crate megam_api;


use std::env;
use std::io;
use std::io::prelude::*;
use self::megam_api::util::accounts::Account;
use self::megam_api::util::accounts::Success;
//use megam_api::util::accounts::Error;

use turbo;
use turbo::util::{CliResult, CliError, Config};
use meg::ops;



#[derive(RustcDecodable)]
pub struct Options {
arg_email: String,
}

pub const USAGE: &'static str = "
Usage:
    meg account [options] <email>

Options:
    -h, --help               Print this message
    --create                  Provide an email to create a new account
    --show                   Provide an email to show the account
    -v, --verbose            Use verbose output
";


pub fn execute(options: Options, _: &Config) -> CliResult<Option<()>> {
    println!("executing; cmd=meg-version; args={:?}", env::args().collect::<Vec<_>>());


    let opts = Account {
        first_name:     format!("{}", "Megam"),
        last_name:      format!("{}", "Systems"),
        phone:          format!("{}", "00"),
        email:          options.arg_email,
        api_key:        format!("{}", "apikey007"),
        password:       format!("{}", "password"),
        authority:      format!("{}", "password1"),
        password_reset_key: format!("{}", "somekey"),
        password_reset_sent_at: format!("{}", "somesentat"),
    };
    let vec = env::args().collect::<Vec<_>>();
    for x in vec.iter() {
        if x == "--create" {

        let out = opts.create();   //.map(|_| None).map_err(|err| {
        //CliError::from_boxed(err, 101)
        println!("{:?}", out);
        match out {
        Ok(v) => {
            println!("success");
        }
        Err(e) => {
            println!("error parsing header");
        }
      }
         } else if x == "--show" {

        //Show(opts).map(|_| None).map_err(|err| {
        //CliError::from_boxed(err, 101)
    //})
     }
 }

    println!("{:?}", vec[2]);
    println!("{:?}",  env::args().collect::<Vec<_>>());

    //match vec {
    //     "--create" => println!("Create--->"),
    //     "--show"   => println!("Show--->"),
    //}
return Ok(None)

}
