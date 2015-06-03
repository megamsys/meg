
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

use megam_api::api::Api;
use self::megam_api::util::accounts::Account;
use self::rustc_serialize::json;
use util::header_hash as head;



#[derive(PartialEq, Clone, Debug)]
pub struct Showoptions {
pub email: String,
}

impl Showoptions {


   pub fn show(&self) {

       let mut opts = Account::new();
       let mut apiObj = head::apiObj().unwrap();
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
