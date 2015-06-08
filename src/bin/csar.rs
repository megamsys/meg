extern crate meg;
extern crate log;

use std::env;
use log::*;
use turbo::util::{CliResult, Config};
use self::meg::ops::meg_csar_create as csar_create;
use self::meg::ops::meg_csar_list as list;
use self::meg::ops::meg_csar_push as push;



#[derive(RustcDecodable)]
struct Options {
    flag_create: String,
    flag_push: String,
    flag_verbose: bool,
}

pub const USAGE: &'static str = "
Usage:
    meg csar [options]


Options:
    -h, --help              Print this message
    --create CSAR           Creates a CSAR entry
    --push ID               Pushes a CSAR entry
    --list                  Lists all existing CSAR
    -v, --verbose           Use verbose output
";

pub fn execute(options: Options, config: &Config) -> CliResult<Option<()>> {
    debug!("executing; cmd=meg-version; args={:?}", env::args().collect::<Vec<_>>());
    config.shell().set_verbose(options.flag_verbose);
    let vec = env::args().collect::<Vec<_>>();
    for x in vec.iter() {
        if x == "--create" {
        let mut csar: csar_create::CsarCoptions  = csar_create::CreateCSAR::new();
        csar.file = options.flag_create.clone();
        csar.create();
        }

        else if x == "--list" {
            let list_csars = list::Csaroptions;
            list_csars.list();
        }

        else if x == "--push" {

             let mut c_push: push::Csaroptions  = push::PushCsar::new();
             c_push.id = options.flag_push.clone();
             c_push.push();


        }
    }
    Ok(None)
}
