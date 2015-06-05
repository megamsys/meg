extern crate curl;

use std::env;

use turbo::util::{CliResult, Config};

pub const USAGE: &'static str = "
Get some help with a meg command.
Usage:
    meg ahoy
    meg ahoy -h | --help
";

#[derive(RustcDecodable)]
struct Options;


pub fn execute(_: Options, _: &Config) -> CliResult<Option<()>> {
    println!("executing; cmd=meg-version; args={:?}", env::args().collect::<Vec<_>>());

    let code =  self::ahoy();
    //    match self::ahoy() {
    //        Some("201") => println!("Megam is up and running!"),
    //        None => println!("Megam is down!")
   //        }

        if code == "201" {
            println!("PONG Megam is up!!");
            //let s = 1;
            // unsafe { main(s) };
        } else {
            println!("Megam is down! Contact support@megam.io");
        }
        Ok(None)
}


pub fn ahoy() -> String {

      let res = curl::http::handle()
      .get("http://api.megam.io")
      .exec().unwrap();
       let x = res.get_code().to_string();
      return x
}
