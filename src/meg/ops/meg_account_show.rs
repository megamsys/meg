
extern crate megam_api;
extern crate rand;
extern crate rustc_serialize;
extern crate term_painter;
extern crate toml;

use self::rustc_serialize::base64::{ToBase64, STANDARD};
use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::marker::Copy;
use std::clone::Clone;


use self::rand::{OsRng, Rng};

use self::term_painter::ToStyle;
use self::term_painter::Color::*;
use self::term_painter::Attr::*;

use self::megam_api::api::Api;
use self::megam_api::util::accounts::Account;
use self::rustc_serialize::json;
use self::megam_api::api::Options as api_options;
use util::parse_toml::Configz;



#[derive(PartialEq, Clone, Debug)]
pub struct Showoptions {
pub email: String,
}

impl Showoptions {


   pub fn show(&self) {
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
       let mut opts = Account::new();
     let out = opts.show(json::encode(&apiObj).unwrap());
      match out {
        Ok(v) => {
         println!("{}",
     Green.bold().paint("Showing your account! "));

}
    Err(e) => {
        println!("{}",
        Red.bold().paint("Oops! are you sure you registered?. "));
        println!("{:?}",
        Red.bold().paint(e));
    }}
   }
}


impl ShowAcc for Showoptions {
    // Replace `Self` with the implementor type: `Account`
    fn new() -> Showoptions {
        Showoptions { email: "".to_string() }
    }

}

pub trait ShowAcc {
    fn new() -> Self;
}
