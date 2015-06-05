
extern crate megam_api;
extern crate rand;
extern crate rustc_serialize;
extern crate term_painter;
extern crate toml;

use self::term_painter::ToStyle;
use self::term_painter::Color::*;

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
          println!("{}",
          Green.bold().paint("Showing your account! "));
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
