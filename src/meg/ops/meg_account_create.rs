
extern crate megam_api;
extern crate rand;
extern crate rustc_serialize;
extern crate term_painter;


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


#[derive(PartialEq, Clone, Debug)]
pub struct Createoptions {
pub email: String,
}



impl Createoptions {


   pub fn create(&self) {
       
    let apiObj = api_options {
        Email: "c@b.com".to_string(),
        Apikey: "firsttest".to_string(),
        Host: "http://localhost:9000".to_string(),
        Version: "/v2".to_string(),
        };
        println!("{:?}", json::encode(&apiObj).unwrap());

        let mut opts = Account::new();

         opts.email = self.email.clone();

    let mut rng = match OsRng::new() {
            Ok(g) => g,
            Err(e) => panic!("Failed to obtain OS RNG: {}", e)
        };
        let mut config = STANDARD;
        let mut num:String = rng.next_u64().to_string();
        let mut api_key:String = num.as_bytes().to_base64(config);

     opts.api_key = api_key.to_string();
     let out = opts.create(json::encode(&apiObj).unwrap());
      match out {
        Ok(v) => {
         println!("{}",
     Green.bold().paint("Hurray!! Account is created! "));
     let mut email = &opts.email.to_string();
     let mut api_key = &api_key;

     createFile(email, api_key)
}
    Err(e) => {
        println!("{}",
        Red.bold().paint("Oops! account was not created. "));
        println!("{:?}",
        Red.bold().paint(e));
    }}
   }
}

pub fn createFile(e: &String, a: &String) {
    let path = Path::new("/home/yeshwanth/megam.toml");
            let mut options = OpenOptions::new();
            options.write(true).create(true);
            let file = match options.open(path) {
                Ok(file) => file,
                Err(..) => panic!("Something is wrong!"),
             };
             let data = format!("[account]\n\nemail = {:?}\napi_key = {:?}", e, a);
             println!("{}",
             Blue.paint("'megam.toml' file is created in your home directory"));

          let mut writer = BufWriter::new(&file);
          writer.write(data.as_bytes());


}

impl CreateAcc for Createoptions {
    // Replace `Self` with the implementor type: `Account`
    fn new() -> Createoptions {
        Createoptions { email: "".to_string() }
    }

}

pub trait CreateAcc {
    fn new() -> Self;
}
