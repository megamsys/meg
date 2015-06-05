extern crate meg;

use std::env;

use turbo::util::{CliResult, Config};
use self::meg::ops::meg_csar_create as csar_create;
use self::meg::ops::meg_csar_list as list;
use self::meg::ops::meg_csar_push as push;



#[derive(RustcDecodable)]
struct Options {
    arg_file:    String,

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

    let vec = env::args().collect::<Vec<_>>();
    for x in vec.iter() {
        if x == "--create" {
        let mut csar: csar_create::CsarCoptions  = csar_create::CreateCSAR::new();
        csar.file = options.arg_file.clone();
        csar.create();
        }

        else if x == "--list" {
            let list_csars = list::Csaroptions;
            list_csars.list();
        }

        else if x == "--push" {

             let mut c_push: push::Csaroptions  = push::PushCsar::new();
             c_push.id = options.arg_file.clone();
             c_push.push();


        }
    }
    Ok(None)
}
