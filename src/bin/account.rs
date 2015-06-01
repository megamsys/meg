extern crate megam_api;
extern crate rand;
extern crate rustc_serialize;

use self::rustc_serialize::base64::{ToBase64, STANDARD};
use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::BufWriter;
use self::rand::{OsRng, Rng};

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
    --create              Provide an email to create a new account
    --show                   Provide an email to show the account
    -v, --verbose            Use verbose output
";


pub fn execute(options: Options, _: &Config) -> CliResult<Option<()>> {
    println!("executing; cmd=meg-account; args={:?}", env::args().collect::<Vec<_>>());


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
            println!("{:?}", v);
            let mut email = &opts.email;

            let mut rng = match OsRng::new() {
                    Ok(g) => g,
                    Err(e) => panic!("Failed to obtain OS RNG: {}", e)
                };
                let mut config = STANDARD;
                let mut num:String = rng.next_u64().to_string();
                let mut api_key:String = num.as_bytes().to_base64(config);
                let mut api_key = &api_key;

            createFile(email, api_key)

        }
        Err(e) => {
            println!("error parsing header");
        }
      }
         } else if x == "--show" {
     }
 }

return Ok(None)

}

pub fn createFile(e: &String, a: &String) {
    let path = Path::new("/home/yeshwanth/megam.toml");
            let mut options = OpenOptions::new();
            options.write(true).create(true);
            //let file: Result<File, Error> = options.open(path);
            let file = match options.open(path) {
                Ok(file) => file,
                Err(..) => panic!("Something is wrong!"),
             };
             let data = format!("email = {:?}\napi_key = {:?}", e, a);
             println!("{:?}", data);
          let mut writer = BufWriter::new(&file);
          writer.write(data.as_bytes());


}
