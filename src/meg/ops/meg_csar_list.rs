extern crate megam_api;
extern crate term_painter;
extern crate toml;
extern crate rustc_serialize;

use std::path::Path;

use self::megam_api::api::Api;
use self::megam_api::api::Options as api_options;
use self::megam_api::util::csars::Csar;
use self::rustc_serialize::json;

use self::term_painter::ToStyle;
use self::term_painter::Color::*;
use self::term_painter::Attr::*;


use util::parse_toml::Configz;
use util::header_hash as head;


pub struct Csaroptions;

impl Csaroptions {

    pub fn list(&self) {

        let mut opts = Csar::new();
        let mut apiObj = head::apiObj().unwrap();
        let out = opts.list(json::encode(&apiObj).unwrap());


         match out {
            Ok(v) => {
                println!("{:?}", v);
            }
            Err(e) => {
                println!("Not listing --->>>");
            }
         }}
    }
