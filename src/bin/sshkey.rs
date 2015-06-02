extern crate megam_api;
extern crate term_painter;
extern crate meg;
extern crate rustc_serialize;
//extern crate core;


use toml;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{BufReader,BufRead};
use std::str;
use std::path::Path;


use self::term_painter::ToStyle;
use self::term_painter::Color::*;
use self::term_painter::Attr::*;



use turbo;
use turbo::util::{CliResult, Config, TurboResult};

use self::megam_api::api::Api;
use self::megam_api::api::Options as api_options;
use self::megam_api::util::sshkeys::SSHKey;
use self::rustc_serialize::json;

use self::meg::util::parse_toml::Configz;



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


   let mut opts = SSHKey::new();

        let vec = env::args().collect::<Vec<_>>();
    for x in vec.iter() {
       if x == "--create" {

       } else if x == "--list" {
            println!("{}",
            Green.bold().paint("Listing SSHKey"));

            let mut path = Path::new("/home/yeshwanth/megam.toml").to_str().unwrap();
            let we = Configz { rand: "sample".to_string()};
            let data = we.load(path);

            let value: toml::Value = data.unwrap();
            let email = value.lookup("account.email").unwrap();
            let api_key = value.lookup("account.api_key").unwrap();

            let apiObj = api_options {
            Email: email.to_string(),
            Apikey: api_key.to_string(),
            Host: "http://localhost:9000".to_string(),
            Version: "/v2".to_string(),
            };
            println!("{:?}", json::encode(&apiObj).unwrap());

            let out = opts.list(json::encode(&apiObj).unwrap());

            match out {
                Ok(v) => {
                    println!("{:?}", v);
                }

                Err(e) => {
                    println!("{:?}", e);
                    println!("{}",
                    Red.bold().paint("Error: Not able to list"));

                    }
               };
        }
     }
 return Ok(None)
}
