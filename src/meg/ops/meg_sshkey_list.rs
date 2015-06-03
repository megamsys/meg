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


pub struct SSHkeyoption;


impl SSHkeyoption {

   pub fn list(&self) {


        let mut path = Path::new("/home/yeshwanth/megam.toml").to_str().unwrap();
        let we = Configz { rand: "sample".to_string()};
        let data = we.load(path);

        let value: toml::Value = data.unwrap();
        let email = value.lookup("account.email").unwrap().as_str().unwrap();
        let api_key = value.lookup("account.api_key").unwrap().as_str().unwrap();


       let apiObj = api_options {
       Email: email.to_string(),
       Apikey: api_key.to_string(),
       Host: "http://localhost:9000".to_string(),
       Version: "/v2".to_string(),
        };



      println!("{:?}", json::encode(&apiObj).unwrap());
      let mut opts = SSHKey::new();

      let out = opts.list(json::encode(&apiObj).unwrap());

    match out {
       Ok(v) => {
        println!("{:?}", v);
    }

    Err(e) => {
        println!("{:?}", e);
        println!("{}",
        Red.bold().paint("Error: Not able to list"));
    }};
   }
}
