#![feature(fs, fs_ext, path_ext, fs_time, fs_walk)]

extern crate rustc_serialize;
extern crate meg;
//extern crate flate2;
//extern crate git2;
//extern crate hamcrest;
//extern crate tar;
//extern crate term;
//extern crate url;
//extern crate tempdir;

#[macro_use]
extern crate log;

//mod support;
macro_rules! test {
    ($name:ident $expr:expr) => (
        #[test]
        fn $name() {
/*            ::support::paths::setup();
            setup();
*/            $expr;
        }
    )
}

mod test_meg_version;
