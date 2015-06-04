extern crate meg;



use std::env;

use turbo;
use turbo::util::{CliResult, Config};
use self::meg::ops::meg_csar_create as csar_create;

#[derive(RustcDecodable)]
struct Options {
    arg_file:    String,
    flag_list:   bool,
}

pub const USAGE: &'static str = "
Usage:
    meg csar [options] [<file>]

Options:
    -h, --help              Print this message
    --create                Creates a CSAR entry
    --push                  Pushes a CSAR entry
    --list                  Lists all existing CSAR
    -v, --verbose           Use verbose output
";

pub fn execute(options: Options, _: &Config) -> CliResult<Option<()>> {
    println!("executing; cmd=meg-version; args={:?}", env::args().collect::<Vec<_>>());

    println!("{:?}", options.arg_file);

    let vec = env::args().collect::<Vec<_>>();
    for x in vec.iter() {
        if x == "--create" { println!("yes,, creates!");
        let mut csar: csar_create::Csar_Coptions  = csar_create::CreateCSAR::new();
        csar.file = options.arg_file.clone();
        let mut x = csar.create();


        }
        else if x == "--list" {println!("yes,, lists!");}
        else if x == "--push" {println!("yes,, pushes!");}


    }


    Ok(None)
}
