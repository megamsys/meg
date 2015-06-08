
extern crate megam_api;
extern crate rand;
extern crate rustc_serialize;
extern crate term_painter;
extern crate yaml_rust;
extern crate toml;

use std::path::Path;
use util::parse_csar::CConfig;

use self::term_painter::ToStyle;
use self::term_painter::Color::*;

use megam_api::api::Api;
use self::megam_api::util::csars::Csar;
use self::megam_api::util::errors::MegResponse;
use util::header_hash as head;



use self::rustc_serialize::json;
use std::str::from_utf8;


#[derive(PartialEq, Clone, Debug)]
pub struct CsarCoptions {
pub file: String,
}

impl CsarCoptions {

   pub fn create(&self) {

    let  file = from_utf8(self.file.as_bytes()).unwrap();
    let  path = Path::new(file).to_str().unwrap();
    let we = CConfig;
    let data = we.load(path);
       let mut opts = Csar::new();
      let api_call = head::api_call().unwrap();

           opts.desc = data.unwrap_or("Error".to_string());
           let out = opts.create(json::encode(&api_call).unwrap());
           match out {
              Ok(v) => {
               let x = json::decode::<MegResponse>(&v).unwrap();
               println!("{}",
               Green.paint("Successfully created your CSAR"));

              println!("----\n\nCode: {:?}\nMessage: {:?}\n\n----", x.code, x.msg);
           }

           Err(e) => {
               println!("{}",
               Red.bold().paint("Error: Not able to create CSAR."));

           }};
    }
}


impl CreateCSAR for CsarCoptions {
    fn new() -> CsarCoptions {
        CsarCoptions { file: "".to_string() }
    }

}

pub trait CreateCSAR {
    fn new() -> Self;
}
