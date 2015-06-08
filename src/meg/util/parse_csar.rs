
extern crate yaml_rust;
extern crate rustc_serialize;

use std::fs::File;
use std::result::Result as StdResult;

use std::io::Read;

pub struct CConfig;


#[derive(Debug)]
pub struct CliError {
    desc: String
}



macro_rules! cli_opt {
    ($op:expr, $fmt:expr) => (
        match $op {
            Some(val) => val,
            None => return Err(CliError {
                desc: format!($fmt)
            }),
        }
    );
    ($op:expr, $fmt:expr, $($args:tt)*) => (
        match $op {
            Some(val) => val,
            None => return Err(CliError {
                desc: format!($fmt, $($args)*)
            }),
        }
    );
}

macro_rules! cli_try {
    ($op:expr) => (
        try!($op.map_err(|err| CliError::from_err(&err)))
    );
    ($op:expr, $fmt:expr) => (
        try!($op.map_err(|err| CliError {
            desc: format!($fmt, err)
        }))
    );
    ($op:expr, $fmt:expr, $($args:tt)*) => (
        try!($op.map_err(|err| CliError {
           desc: format!($fmt, $($args)*, err)
        }))
    );
}


pub type CliResult<T> = StdResult<T, CliError>;

impl CliError {

}


impl CConfig {
    pub fn load(&self, path: &str) -> Result<String ,CliError>  {
        let mut file = cli_try!(
            File::open(path),
            "failed to open config at `{}`: {}",
            path);
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        Ok(buf)
    }
}
