
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
use util::parse_csar::CConfig;

use std::io::BufReader;

use self::rand::{OsRng, Rng};

use self::term_painter::ToStyle;
use self::term_painter::Color::*;
use self::term_painter::Attr::*;

use megam_api::api::Api;
use self::megam_api::api::Options as api_options;
use self::megam_api::util::csars::Csar;

use self::rustc_serialize::json;
use self::yaml_rust::{YamlLoader, YamlEmitter};
use std::str::from_utf8;

use util::parse_toml::Configz;


#[derive(PartialEq, Clone, Debug)]
pub struct Csar_Coptions {
pub file: String,
}



impl Csar_Coptions {


   pub fn create(&self) {


    let mut file = from_utf8(self.file.as_bytes()).unwrap();
    let mut path = Path::new(file).to_str().unwrap();
    let we = CConfig;
    let data = we.load(path);
     println!("{:?}", data);

//this needs to be moved
     let mut path = Path::new("/home/yeshwanth/megam.toml").to_str().unwrap();
     let we = Configz { rand: "sample".to_string()};
     let userData = we.load(path);
     let value: toml::Value = userData.unwrap();
     let email = value.lookup("account.email").unwrap().as_str().unwrap();
     let api_key = value.lookup("account.api_key").unwrap().as_str().unwrap();
//------->

            let apiObj = api_options {
            Email: email.to_string(),
            Apikey: api_key.to_string(),
            Host: "http://localhost:9000".to_string(),
            Version: "/v2".to_string(),
             };
           println!("{:?}", json::encode(&apiObj).unwrap());
           let mut opts = Csar::new();
           opts.desc = data.unwrap();
           println!("-------------------------?>>>>>------------------------->>>>>");
           let out = opts.create(json::encode(&apiObj).unwrap());
           println!("-------------------------------------------------->>>>>");
           match out {
              Ok(v) => {
               println!("{:?}", v);
           }

           Err(e) => {
               println!("{:?}", e);
               println!("{}",
               Red.bold().paint("Error: Not able to create CSAR"));
           }};
    }
}


impl CreateCSAR for Csar_Coptions {
    // Replace `Self` with the implementor type: `Account`
    fn new() -> Csar_Coptions {
        Csar_Coptions { file: "".to_string() }
    }

}

pub trait CreateCSAR {
    fn new() -> Self;
}
