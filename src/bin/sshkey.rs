extern crate megam_api;

use toml;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{BufReader,BufRead};
use std::str;



use turbo;
use turbo::util::{CliResult, Config};
use self::megam_api::util::sshkeys::SSHKey;
use self::megam_api::util::sshkeys::Success;




#[derive(RustcDecodable)]
struct Options;

pub const USAGE: &'static str = "
Usage:
    meg sshkey [options]


Options:
    -h, --help              Print this message
    --list                  List SSHKeys
";


pub fn execute(_: Options, _: &Config) -> CliResult<Option<()>> {
    println!("executing; cmd=meg-sshkey; args={:?}", env::args().collect::<Vec<_>>());

//read from file
        //let data = ReadFile();


        let opts = SSHKey {
           name:   format!("{}", "Megam"),
           accounts_id: format!("{}", "Systems"),
           path: format!("{}", "00"),
        };
        let vec = env::args().collect::<Vec<_>>();
        for x in vec.iter() {
            if x == "--create" {

            let out = opts.create();
        match out {
        Ok(v) => {
            println!("success");
        }
        Err(e) => {
            println!("Error parsing header");
          }
        }} else if x == "--list" {

            let mut file = match File::open("/home/yeshwanth/megam.toml") {
             Ok(file) => file,
             Err(..)  => panic!("Shoot!"),
         };

             let mut reader = BufReader::new(file);
             //let mut xs: Vec<&String> = Vec::new();
            // for x in 0..2 {
        /*   let mut buffer_string = Vec::new();
            try!(reader.read_to_end(&mut buffer_string));
             println!("{:?}", buffer_string);
             let buffer_string = str::from_utf8(buffer_string);
            let path = "test";
             */
            //let temp = buffer_string;
            //xs.push(buffer_string);
            //let length = temp.len();
            // let value = temp.slice_to( length - 2 );
            //parse_again(&buffer_string, &path);
            //let mut parser = toml::Parser::new(buffer_string);
        //}


    /*  let mut data = Vec::new();
       let x = (file.read_to_end(&mut data));
         println!("{:?}", x);
        // let contents = str::from_utf8(&data);
         parse(&data); */


           }

   }

return Ok(None)

}
/*
pub fn parse(data: &[u8]) -> &str {
    //let data = str::from_utf8(&data);
    //data.unwrap();
     parse_first(&data);
}
/
pub fn parse_first(data: &[u8]) {
    let data = str::from_utf8(&data);
    parse_again(data);
}

pub fn parse_again(data: &str, file: &str) -> Result<toml::Table> {
    let mut parser = toml::Parser::new(&data);

    match parser.parse() {

        Some(data) => println!("Finallyyy------>>"),
        None => {},
    }
    Err();

} */
