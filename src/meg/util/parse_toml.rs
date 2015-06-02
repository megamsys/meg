/*
extern crate toml;
extern crate rustc_serialize;


use std::fs::File;
use std::result::Result as StdResult;
use std::error;
use std::error::Error;
use self::rustc_serialize::Decodable;
use self::rustc_serialize::Decoder;
use self::rustc_serialize::json;
use self::rustc_serialize::json::Json;
use std::io::Read;
use std::io::{BufReader,BufRead};

pub struct Config;


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
    fn from_err(err: &Error) -> CliError {
        CliError {
            desc: err.description().to_string()
        }
    }
}


impl Config {
    pub fn load(path: &str) -> Result<Config, CliError> {
        let mut file = cli_try!(
            File::open(path),
            "failed to open config at `{}`: {}",
            path);
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        let result: Result<toml::Value, Vec<toml::ParserError>> = buf.parse();
        let value: toml::Value = try!(result.map_err(|errs| {
            let mut desc = String::new();
            for err in errs {
                desc.push_str(err.description());
                desc.push('\n');
            }
            CliError { desc: desc }
        }));
        let mut profile_name = "Account";
        let profile = cli_opt!(
            value.lookup(profile_name),
            "no profile found for {}",
            profile_name);
               let mut decoder = toml::Decoder::new(profile.clone());
               Ok(cli_try!(Config::decode(&mut decoder)))
    }
}
*/
