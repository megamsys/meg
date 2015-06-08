
extern crate env_logger;
extern crate rustc_serialize;
extern crate toml;
extern crate turbo;
extern crate meg;
extern crate term_painter;

#[macro_use] extern crate log;

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io;
use std::path::{PathBuf, Path};
use std::process::Command;

use turbo::turbo::{execute_main_without_stdin, handle_error, shell};
use turbo::core::MultiShell;
use turbo::util::{CliError, CliResult, Config};
use meg::util::{lev_distance};
use self::term_painter::Color::*;
use self::term_painter::ToStyle;


#[derive(RustcDecodable)]
#[derive(RustcEncodable)]



struct Flags {
    flag_list: bool,
    flag_verbose: bool,
    arg_command: String,
    arg_args: Vec<String>,
}


const USAGE: &'static str = "

Megam command line

Usage:
    meg <command> [<args>...]
    meg [options]

Options:
    -h, --help       Display this message
    version          Print version info and exit
    --list           List installed commands
    -v, --verbose    Use verbose output

 meg commands are:
    ahoy       Ping the status of megam.
    account    Create an account with megam.
    sshkey     Create SSHKey with megam.
    csar       Create apps/services & torpedos


See 'meg help <command>' for more information on a specific command.
";

fn main() {
    env_logger::init().unwrap();
    execute_main_without_stdin(execute, true, USAGE);
}

macro_rules! each_subcommand{ ($mac:ident) => ({
    $mac!(help);
    $mac!(ahoy);
    $mac!(account);
    $mac!(sshkey);
    $mac!(csar);
    $mac!(version);

}) }

/**
  The top-level `cargo` command handles configuration and project location
  because they are fundamental (and intertwined). Other commands can rely
  on this top-level information.
*/
fn execute(flags: Flags, config: &Config) -> CliResult<Option<()>> {
    config.shell().set_verbose(flags.flag_verbose);
    if flags.flag_list {
        println!("{}",
        Green.paint("Installed commands:"));
        for command in list_commands().into_iter() {
            println!("{}", command);
        };
        return Ok(None)
    }

    let args = match &flags.arg_command[..] {
        // For the commands `meg` and `meg help`, re-execute ourselves as
        // `meg -h` so we can go through the normal process of printing the
        // help message.
        "" | "help" if flags.arg_args.is_empty() => {
            config.shell().set_verbose(true);
            let args = &["meg".to_string(), "-h".to_string()];
            let r = turbo::turbo::call_main_without_stdin(execute, config, USAGE, args,
                                                   false);

            turbo::turbo::process_executed(r, &mut config.shell());
            return Ok(None)
        }

        // For `meg help -h` and `meg help --help`, print out the help
        // message for `meg help`
        "help" if flags.arg_args[0] == "-h" ||
                  flags.arg_args[0] == "--help" => {
            vec!["meg".to_string(), "help".to_string(), "-h".to_string()]
        }

        // For `meg help foo`, print out the usage message for the specified
        // subcommand by executing the command with the `-h` flag.
        "help" => {
            vec!["meg".to_string(), flags.arg_args[0].clone(),
                 "-h".to_string()]
        }

        // For all other invocations, we're of the form `meg foo args...`. We
        // use the exact environment arguments to preserve tokens like `--` for
        // example.



        "account" if flags.arg_args.is_empty() => {
            config.shell().set_verbose(true);
            let args = &["meg".to_string(), "help".to_string(), "account".to_string()];
            let r = turbo::turbo::call_main_without_stdin(execute, config, USAGE, args,
                                                   false);

            turbo::turbo::process_executed(r, &mut config.shell());
            return Ok(None)
        }


        "sshkey" if flags.arg_args.is_empty() => {
            config.shell().set_verbose(true);
            let args = &["meg".to_string(), "help".to_string(), "sshkey".to_string()];
            let r = turbo::turbo::call_main_without_stdin(execute, config, USAGE, args,
                                                           false);

            turbo::turbo::process_executed(r, &mut config.shell());
            return Ok(None)
                }

        "csar" if flags.arg_args.is_empty() => {
                    config.shell().set_verbose(true);
                    let args = &["meg".to_string(), "help".to_string(), "csar".to_string()];
                    let r = turbo::turbo::call_main_without_stdin(execute, config, USAGE, args,
                                                                   false);

                    turbo::turbo::process_executed(r, &mut config.shell());
                    return Ok(None)
                        }



        _ => env::args().collect(),
    };

    macro_rules! cmd{ ($name:ident) => (
        if args[1] == stringify!($name).replace("_", "-") {
            mod $name;
            config.shell().set_verbose(true);
            let r = turbo::turbo::call_main_without_stdin($name::execute, config,
                                                   $name::USAGE,
                                                   &args,
                                                   false);
            turbo::turbo::process_executed(r, &mut config.shell());
            return Ok(None)
        }
    ) }
    each_subcommand!(cmd);

    execute_subcommand(&args[1], &args, &mut config.shell());
    Ok(None)
}

fn find_closest(cmd: &str) -> Option<String> {
    let cmds = list_commands();
    // Only consider candidates with a lev_distance of 3 or less so we don't
    // suggest out-of-the-blue options.
    let mut filtered = cmds.iter().map(|c| (lev_distance(&c, cmd), c))
                                  .filter(|&(d, _)| d < 4)
                                  .collect::<Vec<_>>();
    filtered.sort_by(|a, b| a.0.cmp(&b.0));

    if filtered.len() == 0 {
        None
    } else {
        Some(filtered[0].1.to_string())
    }
}

fn execute_subcommand(cmd: &str, args: &[String], shell: &mut MultiShell) {
    let command = match find_command(cmd) {
        Some(command) => command,
        None => {
            let msg = match find_closest(cmd) {
                Some(closest) => format!("No such subcommand\n\n\t\
                                          Did you mean `{}`?\n", closest),
                None => "No such subcommand".to_string()
            };
            return handle_error(CliError::new(&msg, 127), shell)
        }
    };
    match Command::new(&command).args(&args[1..]).status() {
        Ok(ref status) if status.success() => {}
        Ok(ref status) => {
            match status.code() {
                Some(code) => handle_error(CliError::new("", code), shell),
                None => {
                    let msg = format!("subcommand failed with: {}", status);
                    handle_error(CliError::new(&msg, 101), shell)
                }
            }
        }
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            handle_error(CliError::new("No such subcommand", 127), shell)
        }
        Err(err) => {
            let msg = format!("Subcommand failed to run: {}", err);
            handle_error(CliError::new(&msg, 127), shell)
        }
    }
}

/// List all runnable commands. find_command should always succeed
/// if given one of returned command.
fn list_commands() -> BTreeSet<String> {
    let command_prefix = "meg-";
    let mut commands = BTreeSet::new();
    for dir in list_command_directory().iter() {
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            _ => continue
        };
        for entry in entries {
            let entry = match entry { Ok(e) => e, Err(..) => continue };
            let entry = entry.path();
            let filename = match entry.file_name().and_then(|s| s.to_str()) {
                Some(filename) => filename,
                _ => continue
            };
            if filename.starts_with(command_prefix) &&
               filename.ends_with(env::consts::EXE_SUFFIX) &&
               is_executable(&entry) {
                let command = &filename[
                    command_prefix.len()..
                    filename.len() - env::consts::EXE_SUFFIX.len()];
                commands.insert(command.to_string());
            }
        }
    }

    macro_rules! add_cmd{ ($cmd:ident) => ({
        commands.insert(stringify!($cmd).replace("_", "-"));
    }) }
    each_subcommand!(add_cmd);
    commands
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    //use std::os::unix;
 //use std::sys::ext;
    //fs::metadata(path).map(|m| {
//        m.permissions() == 0o001
//    }).unwrap_or(false)
return true
}
#[cfg(windows)]
fn is_executable(path: &Path) -> bool {
    fs::metadata(path).map(|m| m.is_file()).unwrap_or(false)
}

/// Get `Command` to run given command.
fn find_command(cmd: &str) -> Option<PathBuf> {
    let command_exe = format!("meg-{}{}", cmd, env::consts::EXE_SUFFIX);
    let dirs = list_command_directory();
    let mut command_paths = dirs.iter().map(|dir| dir.join(&command_exe));
    command_paths.find(|path| fs::metadata(&path).is_ok())
}

/// List candidate locations where subcommands might be installed.
fn list_command_directory() -> Vec<PathBuf> {
    let mut dirs = vec![];
    if let Ok(mut path) = env::current_exe() {
        path.pop();
        dirs.push(path.join("../lib/meg"));
        dirs.push(path);
    }
    if let Some(val) = env::var_os("PATH") {
        dirs.extend(env::split_paths(&val));
    }
    dirs
}
