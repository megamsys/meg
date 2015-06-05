
extern crate megam_api;
extern crate rand;
extern crate rustc_serialize;
extern crate term_painter;
extern crate yaml_rust;
extern crate toml;

use self::term_painter::ToStyle;
use self::term_painter::Color::*;

use megam_api::api::Api;
use self::megam_api::util::csars::Csar;
use self::megam_api::util::errors::MegResponse;
use self::rustc_serialize::json;
use util::header_hash as head;
use std::str::from_utf8;


#[derive(PartialEq, Clone, Debug)]
pub struct Csaroptions {
 pub id: String,
}


impl Csaroptions {

    pub fn push(&self) {
        let data = from_utf8(self.id.as_bytes()).unwrap();
        let  opts = Csar::new();
        let  api_call = head::api_call().unwrap();
        let out = opts.push(json::encode(&api_call).unwrap(), data.to_string());
         match out {
           Ok(v) => {
               let x = json::decode::<MegResponse>(&v).unwrap();
               println!("{}",
               Green.paint("Successfully pushed your CSAR"));

              println!("----\nCode: {:?}\nMessage: {:?}\n----", x.code, x.msg);
        }
          Err(e) => {
           println!("{}",
           Red.bold().paint("\nOops! some error! coudnt push the csar, contact support@megam.io\n"));
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
