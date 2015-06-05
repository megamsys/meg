extern crate megam_api;
extern crate term_painter;
extern crate toml;
extern crate rustc_serialize;
extern crate megam_rustyprint;

use self::megam_rustyprint::Printer;
use self::megam_api::api::Api;
use self::megam_api::util::csars::Csar;
use self::rustc_serialize::json;


use self::term_painter::ToStyle;
use self::term_painter::Color::*;

use util::header_hash as head;


pub struct Csaroptions;

impl Csaroptions {

    pub fn list(&self) {

        let opts = Csar::new();
        let api_call = head::api_call().unwrap();
        let out = opts.list(json::encode(&api_call).unwrap());


         match out {
            Ok(v) => {
                println!("{}",
                 Green.bold().paint("CSARs\n"));
                 let mut a = Printer::new();
                 let mut header = Vec::new();
                 header.push("Id".to_string());
                 header.push("Description".to_string());
                 header.push("Created at".to_string());
                 a.set_header(header);
                  let mut parent = Vec::new();

                 for x in v.iter() {

                     let mut child = Vec::new();
                     child.push(x.link.to_string());
                     child.push(x.desc.to_string());
                     child.push(x.created_at.to_string());
                      parent.push(child);
                 }
                   a.set_body(parent);

                   println!("{:?}", a);
            }
            Err(e) => {
                println!("{}",
                 Red.bold().paint("Not listing your CSARs You sure you created them? . Contact support@megam.io"));

            }
         }}
    }
