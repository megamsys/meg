
extern crate megam_api;
extern crate rand;
extern crate rustc_serialize;
extern crate term_painter;
extern crate yaml_rust;
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
use self::megam_api::util::csars::Csar;
use self::rustc_serialize::json;
use util::header_hash as head;
use util::parse_csar::CConfig;
use std::str::from_utf8;


#[derive(PartialEq, Clone, Debug)]
pub struct Csaroptions {
 pub id: String,
}


impl Csaroptions {

    pub fn push(&self) {
        let data = from_utf8(self.id.as_bytes()).unwrap();
        let mut opts = Csar::new();
        let mut apiObj = head::apiObj().unwrap();
        println!("--0-0-0-0-0-0-0-0--{:?}", data);
        let out = opts.push(json::encode(&apiObj).unwrap(), data.to_string());
         match out {
           Ok(v) => {
           println!("{}",
           Green.bold().paint("Pushed it... "));
        }
          Err(e) => {
           println!("{}",
           Red.bold().paint("Oops! some error! coudnt push the csar, contact support@megam.io. "));
           println!("{:?}",
           Red.bold().paint(e));
         }};
}
}


impl PushCsar for Csaroptions {
    fn new() -> Csaroptions {
        Csaroptions { id: "".to_string() }
    }
}

pub trait PushCsar {
    fn new() -> Self;
}
