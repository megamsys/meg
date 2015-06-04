extern crate megam_api;
extern crate term_painter;
extern crate toml;
extern crate rustc_serialize;

use std::path::Path;

use self::megam_api::api::Api;
use self::megam_api::api::Options as api_options;
use self::megam_api::util::sshkeys::SSHKey;
use self::rustc_serialize::json;

use self::term_painter::ToStyle;
use self::term_painter::Color::*;
use self::term_painter::Attr::*;

use util::parse_toml::Configz;
use util::header_hash as head;


pub struct SSHkeyoption;


impl SSHkeyoption {

   pub fn list(&self) {


      let mut opts = SSHKey::new();
      let mut apiObj = head::apiObj().unwrap();
      let out = opts.list(json::encode(&apiObj).unwrap());
     println!("---------------------------{:?}", out);
    match out {
       Ok(v) => {
           println!("{:?}",
           Green.bold().paint(v));
           //println!("{:?}", v.as_slice());
    }

    Err(e) => {
        println!("{:?}", e);
        println!("{}",
        Red.bold().paint("Error: Not able to list"));
    }};
   }
}
