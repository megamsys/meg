

extern crate megam_api;
extern crate rand;
extern crate rustc_serialize;
extern crate term_painter;
extern crate toml;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::marker::Copy;
use std::clone::Clone;
use std::result::Result as StdResult;
use std::error::Error;

use util::parse_toml::Configz;
use self::megam_api::api::Options as api_options;


#[derive(Debug)]
pub struct CliError {
    desc: String
}


pub type CliResult<T> = StdResult<T, CliError>;

impl CliError {

}



pub fn apiObj() -> Result<api_options, CliError> {


let mut x = env::home_dir().unwrap();
let mut y = x.to_str().unwrap();
let setPath = format!("{}/megam.toml", y.to_string());

let mut path = Path::new(&setPath).to_str().unwrap();
let we = Configz { rand: "sample".to_string()};
let data = we.load(path);

let value: toml::Value = data.unwrap();
let email = value.lookup("account.email").unwrap().as_str().unwrap();
let api_key = value.lookup("account.api_key").unwrap().as_str().unwrap();



 let mut apiObjt: api_options = api_options {
Email: email.to_string(),
Apikey: api_key.to_string(),
Host: "fefe".to_string(),
Version: "asd".to_string(),
};
 Ok(apiObjt)
 }
