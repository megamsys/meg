
use turbo::util::{CliResult, CliError, Config};

#[derive(RustcDecodable)]
struct Options;

pub const USAGE: &'static str = "
Get some help with a meg command.
Usage:
    meg help <command>
    meg help -h | --help
Options:
    -h, --help          Print this message
";

pub fn execute(_: Options, _: &Config) -> CliResult<Option<()>> {
    // This is a dummy command just so that `meg help help` works.
    // The actual delegation of help flag to subcommands is handled by the
    // meg command.
    Err(CliError::new("Help command should not be executed directly.", 101))

}
