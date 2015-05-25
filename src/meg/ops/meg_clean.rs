use std::default::Default;
use std::io::prelude::*;

use util::{TurboResult};

pub struct CleanOptions<'a, 'b: 'a> {
    pub spec: Option<&'a str>,
    pub target: Option<&'a str>,
    pub config: &'a Config<'b>,
}

/// Cleans the project from build artifacts.
pub fn clean(manifest_path: &Path, opts: &CleanOptions) -> TurboResult<()> {
    Ok(())
}
