extern crate megam_api;
extern crate term_painter;
extern crate toml;
extern crate rustc_serialize;

use self::megam_api::api::Api;
use self::megam_api::util::csars::Csar;
use self::rustc_serialize::json;

use util::header_hash as head;


pub struct Csaroptions;

impl Csaroptions {

    pub fn list(&self) {

        let opts = Csar::new();
        let api_call = head::api_call().unwrap();
        let out = opts.list(json::encode(&api_call).unwrap());


         match out {
            Ok(v) => {
                println!("{:?}", v);
            }
            Err(e) => {
                println!("Not listing --->>>");
                println!("Error: {:?}", e);

            }
         }}
    }
