
extern crate megam_api;
extern crate rand;
extern crate rustc_serialize;
extern crate term_painter;
extern crate toml;
extern crate megam_rustyprint;

use self::megam_rustyprint::Printer;


use self::term_painter::ToStyle;
use self::term_painter::Color::*;
use self::megam_api::util::errors::MegResponse;


use megam_api::api::Api;
use self::megam_api::util::accounts::Account;
use self::rustc_serialize::json;
use util::header_hash as head;



#[derive(PartialEq, Clone, Debug)]
pub struct Showoptions {
pub email: String,
}

impl Showoptions {


   pub fn show(&self) {

       let opts = Account::new();
       let api_call = head::api_call().unwrap();
       let out = opts.show(json::encode(&api_call).unwrap());
        match out {
          Ok(v) => {
              println!("{:?}", v);
              println!("{}",
               Green.bold().paint("Your account \n"));
               let mut a = Printer::new();
               let mut header = Vec::new();
               header.push("Name".to_string());
               header.push("Email".to_string());
               header.push("API Key".to_string());
               a.set_header(header);
                let mut parent = Vec::new();

                //let x = json::decode::<Account>(&v).unwrap();

                   let mut child = Vec::new();
                   child.push(v.first_name.to_string());
                   child.push(v.email.to_string());
                   child.push(v.api_key.to_string());
                    parent.push(child);

                 a.set_body(parent);

                 println!("{:?}", a);

       }
         Err(e) => {
          println!("{}",
          Red.bold().paint("Oops! are you sure you registered?. "));
          println!("{:?}",
          Red.bold().paint(e));
        }}
      }
    }


impl ShowAcc for Showoptions {
    fn new() -> Showoptions {
        Showoptions { email: "".to_string() }
    }

}

pub trait ShowAcc {
    fn new() -> Self;
}
