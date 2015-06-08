extern crate curl;
extern crate term_painter;
extern crate rustc_serialize;
extern crate toml;

use std::path::Path;

use std::env;

use turbo::util::{CliResult, Config};
use self::term_painter::Color::*;
use self::term_painter::ToStyle;
use self::rustc_serialize::json;
use meg::util::parse_toml::Configz;
//use self::curl::ffi::err::ErrCode;


pub const USAGE: &'static str = "
Get some help with a meg command.
Usage:
    meg ahoy
    meg ahoy -h | --help
";

#[derive(RustcDecodable)]
struct Options;


pub fn execute(_: Options, _: &Config) -> CliResult<Option<()>> {
    //println!("executing; cmd=meg-version; args={:?}", env::args().collect::<Vec<_>>());
    let code =  self::ahoy();
     //  match self::ahoy() {
    //        Ok(v) => println!("Megam is up and running!"),
    //        Err(e) => println!("Megam is down! {:?}", e)
    //       }

        if code == "200" {
            println!("{}",
            Green.paint("PONG! Megam is up."));

        } else {
            println!("{}",
            Red.paint("Megam is down!"));
            println!("{}",
            Blue.paint("Please check whether your host is setup in /home/.megam/megam.toml"));
        }
        Ok(None)
}

//#[derive(Debug)]
//pub struct ErrCode {
//    desc: String
//}

pub fn ahoy() -> String {

    let hme = env::home_dir().unwrap();
    let y = hme.to_str().unwrap();
    let set_path = format!("{}/.megam/megam.toml", y.to_string());

    let path = Path::new(&set_path).to_str().unwrap();
    let we = Configz { rand: "sample".to_string()};
    let data = we.load(path);
    let value: toml::Value = data.unwrap();
    let host = value.lookup("api.host").unwrap().as_str().unwrap();

       let res = curl::http::handle()
      .get(host)
      .exec();

       let x = res.unwrap().get_code().to_string();
      return x
}
/*
fn error_return(result: Result<curl::http::response::Response, ErrCode>) -> Result<curl::http::response::Response, ErrCode> {
   match result {
        Ok(n) => {
						return Ok(n)
				},
        Err(FailOne) => return Err(FailOne),
    }
} */
