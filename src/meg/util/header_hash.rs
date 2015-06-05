

extern crate megam_api;
extern crate rand;
extern crate rustc_serialize;
extern crate term_painter;
extern crate toml;

use std::path::Path;
use std::fs::OpenOptions;
use std::result::Result as StdResult;

use util::parse_toml::Configz;
use self::megam_api::api::Options as api_options;


#[derive(Debug)]
pub struct CliError {
    desc: String
}


pub type CliResult<T> = StdResult<T, CliError>;

impl CliError {

}



pub fn api_call() -> Result<api_options, CliError> {

<<<<<<< HEAD

let mut x = env::home_dir().unwrap();
let mut y = x.to_str().unwrap();
let setPath = format!("{}/megam.toml", y.to_string());

let mut path = Path::new(&setPath).to_str().unwrap();
=======
let path = Path::new("/home/yeshwanth/megam.toml").to_str().unwrap();
>>>>>>> origin/master
let we = Configz { rand: "sample".to_string()};
let data = we.load(path);

let value: toml::Value = data.unwrap();
let email = value.lookup("account.email").unwrap().as_str().unwrap();
let api_key = value.lookup("account.api_key").unwrap().as_str().unwrap();
let host = value.lookup("api.host").unwrap().as_str().unwrap();
let version = value.lookup("api.version").unwrap().as_str().unwrap();



 let api_run_hash: api_options = api_options {
Email: email.to_string(),
Apikey: api_key.to_string(),
Host: host.to_string(),
Version: version.to_string(),
};
 Ok(api_run_hash)
 }
