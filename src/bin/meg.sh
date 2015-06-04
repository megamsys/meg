#!/bin/sh
##
## Copyright [2013-2015] [Megam Systems]
##
## Licensed under the Apache License, Version 2.0 (the "License");
## you may not use this file except in compliance with the License.
## You may obtain a copy of the License at
##
## http://www.apache.org/licenses/LICENSE-2.0
##
## Unless required by applicable law or agreed to in writing, software
## distributed under the License is distributed on an "AS IS" BASIS,
## WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
## See the License for the specific language governing permissions and
## limitations under the License.
##

# Temporary files must be carefully deleted on every error path.

set -u # Undefined variables are errors

txtblk='\e[0;30m' # Black - Regular
txtred='\e[0;31m' # Red
txtgrn='\e[0;32m' # Green
txtylw='\e[0;33m' # Yellow
txtblu='\e[0;34m' # Blue
txtpur='\e[0;35m' # Purple
txtcyn='\e[0;36m' # Cyan
txtwht='\e[0;37m' # White
bldblk='\e[1;30m' # Black - Bold
bldred='\e[1;31m' # Red
bldgrn='\e[1;32m' # Green
bldylw='\e[1;33m' # Yellow
bldblu='\e[1;34m' # Blue
bldpur='\e[1;35m' # Purple
bldcyn='\e[1;36m' # Cyan
bldwht='\e[1;37m' # White
unkblk='\e[4;30m' # Black - Underline
undred='\e[4;31m' # Red
undgrn='\e[4;32m' # Green
undylw='\e[4;33m' # Yellow
undblu='\e[4;34m' # Blue
undpur='\e[4;35m' # Purple
undcyn='\e[4;36m' # Cyan
undwht='\e[4;37m' # White
bakblk='\e[40m'   # Black - Background
bakred='\e[41m'   # Red
bakgrn='\e[42m'   # Green
bakylw='\e[43m'   # Yellow
bakblu='\e[44m'   # Blue
bakpur='\e[45m'   # Purple
bakcyn='\e[46m'   # Cyan
bakwht='\e[47m'   # White
txtrst='\e[0m'    # Text Reset


main() {
    assert_cmds
    set_globals
    handle_command_line_args "$@"
}

set_globals() {
    # Environment sanity checks
    assert_nz "$HOME" "\$HOME is undefined"
    assert_nz "$0" "\$0 is undefined"

    # Some constants
    version=0.0.1
    metadata_version=1

    # Find the location of the distribution server
    default_dist_server="http://get.megam.io"
    dist_server="${MEG_DIST_SERVER-$default_dist_server}"

    # The directory on the server containing the dist artifacts
    meg_dist_dir=meg

    # Set up the meg data dir
    meg_dir="${MEG_HOME-$HOME/.megam}"
    assert_nz "$meg_dir" "meg_dir"

    # Install prefix can be set by the environment
    default_prefix="${MEG_PREFIX-$HOME/bin}"

    # Data locations
    version_file="$meg_dir/version"
    temp_dir="$meg_dir/tmp"

    flag_verbose=false
    flag_yes=false

    if [ -n "${MEG_VERBOSE-}" ]; then
	     flag_verbose=true
    fi
}

# Ensures that ~/.megam exists and uses the correct format
initialize_metadata() {


    verbose_say "checking metadata version"


    # Make sure ~/.megam is writable
    if [ -e "$version_file" ]; then
	     local _can_write=true
	     local _probe_file="$meg_dir/write-probe"
	     ignore touch "$_probe_file" 2> /dev/null
	  if [ $? != 0 ]; then
	     _can_write=false
	   else
	      ensure rm "$_probe_file"
	   fi

	if [ "$_can_write" = false ]; then
	    say "$meg_dir is unwritable. it was likely created by a previous meg run under sudo"
	fi

    ensure mkdir -p "$meg_dir"
    meg_dir="$(abs_path "$rustup_dir")"
    assert_nz "$meg_dir" "meg_dir"
  fi
}

handle_command_line_args() {

    local _date=""
    local _prefix="$default_prefix"
    local _help=false

    for arg in "$@"; do
	     case "$arg" in

	      --help )
		      _help=true
		        ;;

	      --verbose)
		      # verbose is a global flag
		        flag_verbose=true
		        ;;


	    -y | --yes)
		# yes is a global flag
		  flag_yes=true
		    ;;

	    --version)
    	echo -e "${bldwht}meg.sh $version${txtrst}"
		  exit 0
		;;

	esac

	if is_value_arg "$arg" "prefix"; then
	    _prefix="$(get_value_arg "$arg")"
	elif is_value_arg "$arg" "date"; then
	    _date="$(get_value_arg "$arg")"
	fi
  done

    if [ "$_help" = true ]; then
	print_help
	exit 0
    fi

  if [ "$flag_yes" = false ]; then
	# Running in interactive mode, check that a tty exists
	check_tty

	# Print the welcome / warning message and wait for confirmation
	print_welcome_message "$_prefix"

	get_tty_confirmation
    fi



    # Make sure our data directory exists and is the right format
    initialize_metadata

    # OK, time to do the things
    local _succeeded=true
	  install_toolchain_from_dist "$_prefix"

   if [ $? != 0 ]; then
	    _succeeded=false
      exit 1
	   fi

}

is_value_arg() {
    local _arg="$1"
    local _name="$2"

    echo "$arg" | grep -q -- "--$_name="
    return $?
}

get_value_arg() {
    local _arg="$1"

    echo "$_arg" | cut -f2 -d=
}



validate_date() {
    local _date="$1"

    case "$_date" in
	[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9] )
	    ;;
	* )
	    err "date must be in YYYY-MM-DD format"
	    ;;
    esac
}

print_welcome_message() {
    local _prefix="$1"

    say "cli installer."


	if [ "$(id -u)" = 0 ]; then
      printf "${txtylw}%s:${txtrst}:${bldcyn}%s${txtrst}\n" "WARNING" "This script appears to be running as root. While it will work";
      printf "${bldcyn}%s${txtrst}\n" "correctly, it is no longer necessary for meg.sh to run as root."
	fi

  printf "${txtcyn}%s:${txtrst}\n" "This script will download the meg cli, and install them to $_prefix";
  printf "${txtcyn}%s${txtrst}\n" "You may uninstall later by deleting  $_prefix"

}


# Updating toolchains

# Returns 0 on success, 1 on error
install_toolchain_from_dist() {
    local _prefix="$1"

    local _potential_oldmeg_bin="$_prefix/bin/meg"
    if [ -e "$_potential_oldmeg_bin" ]; then
	say_err "meg appears to be installed at the destination, $_potential_oldmeg_bin"
	say_err "installing meg will replace the old meg."
    fi

    determine_remote_meg_installer_location  || return 1
    local _remote_meg_installer="$RETVAL"
    assert_nz "$_remote_meg_installer" "remote meg installer"
    verbose_say "remote meg installer location: $_remote_meg_installer"

    local _meg_installer_name="$(basename "$_remote_meg_installer")"
    assert_nz "$_meg_installer_name" "meg installer name"

    # Download and install toolchain
    say "downloading ..."
    download_and_check "$_remote_meg_installer"

    #if [ "$_retval" != 0 ]; then
#	     return 1
  #	  fi
    local _installer_file="$RETVAL"

    assert_nz "$_installer_file" "installer_file"

    local _workdir="$RETVAL_WORK"
    assert_nz "$_workdir" "workdir"
    verbose_say "install tmp dir: $_workdir"

    # There next few statements may all fail independently.
    local _failing=false

    install_toolchain "$_installer_file" "$_workdir" "$_prefix"

    if [ $? != 0 ]; then
	say_err "failed to install meg"
	_failing=true
    fi

say "installed successfully."
printf "$bldwht%s$txtrst: $bldylw%s$txtrst\n" "meg" "change .bashrc PATH="'$PATH'":~/bin";
printf "$bldwht%s$txtrst: $bldylw%s$txtrst\n" "meg" "Refer:  http://docs.megam.io/v1.0/docs/megam_quick_launch";
printf "$txtgrn%s$txtrst\n" "bye :)"

    run rm -R "$_workdir"
    if [ $? != 0 ]; then
	say_err "couldn't delete workdir"
	_failing=true
    fi

  if [ "$_failing" = true ]; then
	return 1
    fi
}

install_toolchain() {
    local _installer="$1"
    local _workdir="$2"
    local _prefix="$3"

    local _installer_dir="$_workdir/$(basename "$_installer" | sed s/.tar.gz$//)"

    # Extract the toolchain
    say "extracting installer"
    run tar xzf "$_installer" -C "$_workdir"
    if [ $? != 0 ]; then
	verbose_say "failed to extract installer"
	return 1
    fi

    # Install the toolchain
    local _toolchain_dir="$_prefix"
    verbose_say "installing meg to '$_prefix'"

    say "installing meg"
    cp $_installer_dir/meg $_prefix

    if [ $? != 0 ]; then
	verbose_say "failed to install meg"
	return 1
    fi

}


# Meg installable location.
determine_remote_meg_installer_location() {
    verbose_say "determining remote meg installer"
    get_remote_installer_location  meg "$meg_dist_dir" || return 1
    RETVAL="$RETVAL"
}


get_remote_installer_location() {
    local _package_name="$1"
    local _dist_dir="$2"

    get_architecture
    local _arch="$RETVAL"
    assert_nz "$_arch" "arch"

	if [ $? = 0 ]; then
		    RETVAL="$dist_server/$_dist_dir/$_package_name-nightly.$_arch.tar.gz"
  else
    err "couldn't find remote installer for '$_arch'. contact support@megam.io."
    return
 fi
}

# Tools
# FIXME: Temp names based on pid need to worry about pid recycling
make_temp_name() {
    local _pid="$$"
    assert_nz "$_pid" "pid"

    local _tmp_number="${NEXT_TMP_NUMBER-0}"
    local _tmp_name="tmp-$_pid-$_tmp_number"
    NEXT_TMP_NUMBER=$((_tmp_number + 1))
    need_ok "failed to create temp number"
    assert_nz "$NEXT_TMP_NUMBER" "NEXT_TMP_NUMBER"
    RETVAL="$_tmp_name"
}

make_temp_dir() {
    ensure mkdir -p "$temp_dir"

    ensure make_temp_name
    local _tmp_name="$temp_dir/$RETVAL"
    ensure mkdir -p "$_tmp_name"
    RETVAL="$_tmp_name"
}

#
get_architecture() {

    verbose_say "detecting architecture"

    local _ostype="$(uname -s)"
    local _cputype="$(uname -m)"

    verbose_say "uname -s reports: $_ostype"
    verbose_say "uname -m reports: $_cputype"

    if [ "$_ostype" = Darwin -a "$_cputype" = i386 ]; then
	# Darwin `uname -s` lies
	if sysctl hw.optional.x86_64 | grep -q ': 1'; then
	    local _cputype=x86_64
	fi
    fi

    case "$_ostype" in

	Linux)
	    local _ostype=linux-gnu
	    ;;


	Darwin)
	    local _ostype=apple-darwin
	    ;;

	MINGW* | MSYS*)
	    err "unimplemented windows arch detection"
	    ;;

	*)
	    err "unrecognized OS type: $_ostype"
	    ;;

    esac

    case "$_cputype" in

	i386 | i486 | i686 | i786 | x86)
            local _cputype=i686
            ;;

	x86_64 | x86-64 | x64 | amd64)
            local _cputype=x86_64
            ;;

	*)
            err "unknown CPU type: $CFG_CPUTYPE"

    esac

    # Detect 64-bit linux with 32-bit userland
    if [ $_ostype = linux-gnu -a $_cputype = x86_64 ]; then
	# $SHELL does not exist in standard 'sh', so probably only exists
	# if configure is running in an interactive bash shell. /usr/bin/env
	# exists *everywhere*.
	local _bin_to_probe="$SHELL"
	if [ -z "$_bin_to_probe" -a -e "/usr/bin/env" ]; then
	    _bin_to_probe="/usr/bin/env"
	fi
	if [ -n "$_bin_to_probe" ]; then
	    file -L "$_bin_to_probe" | grep -q "x86[_-]64"
	    if [ $? != 0 ]; then
		local _cputype=i686
	    fi
	fi
    fi

    local _arch="$_cputype-$_ostype"
    verbose_say "architecture is $_arch"

    RETVAL="$_arch"
}

# Downloads a remote file, its checksum, and signature and verifies them.
# Returns 0 on success. Returns the path to the downloaded file in RETVAL,
# and the path to it's directory in the cache in RETVAL_CACHE.
#
# The caller can decide to remove it from the cache by deleting RETVAL_CACHE.
#
# A return code of *20* indicates a successful short circuit from the
# update hash.
download_and_check() {
    local _remote_name="$1"


    local _remote_basename="$(basename "$_remote_name")"

    make_temp_dir
    local _workdir="$RETVAL"
    assert_nz "$_workdir" "workdir"
    verbose_say "download work dir: $_workdir"

    download_file "$_remote_name" "$_workdir"
    if [ $? != 0 ]; then
      ignore rm -R "$_workdir"
	    return 1
    fi

    RETVAL="$_workdir/$_remote_basename"
    RETVAL_WORK="$_workdir"
}


download_file() {
    local _remote_name="$1"
    local _local_dirname="$2"


    local _remote_basename="$(basename "$_remote_name")"
    assert_nz "$_remote_basename" "remote basename"

    local _local_name="$_local_dirname/$_remote_basename"

    verbose_say "downloading '$_remote_name' to '$_local_name'"
    # Invoke curl in a way that will resume if necessary
	(run cd "$_local_dirname" && run curl -# -C - -f -O "$_remote_name")

    if [ $? != 0 ]; then
	say_err "couldn't download '$_remote_name'"
	return 1
    fi
}


# Verifies that the terminal can be opened or exits
check_tty() {
    # FIXME: This isn't sufficient since it just checks that tty
    # exists, not that it can be read
    if [ ! -e /dev/tty ]; then
	err "running in interactive mode (without -y), but cannot open /dev/tty"
    fi
}

# Waits for a y/n response and exits if n
get_tty_confirmation() {
    local _yn=""
    read -p "Continue? (y/N) " _yn < /dev/tty
    need_ok "failed to read from /dev/tty"

    echo

    if [ "$_yn" != "y" -a "$_yn" != "Y" -a "$_yn" != "yes" ]; then
	say "cancelled."
	exit 0
    fi
}

# Help

print_help() {
echo '
Usage: meg.sh [--verbose]

Options:

     --prefix=<path>                   Install to a specific location (default /home/<user>/bin)
'
}

# Standard utilities

say() {
    printf "${bldwht}meg:${txtrst} ${txtylw}%-s${txtrst}\n" "$1";
}

say_err() {
    say "$1" >&2
}

verbose_say() {
    if [ "$flag_verbose" = true ]; then
	say "$1"
    fi
}

err() {
    say "$1" >&2
    exit 1
}

need_cmd() {
    if ! command -v "$1" > /dev/null 2>&1
    then err "need $1"
    fi
}

need_ok() {
    if [ $? != 0 ]; then err "$1"; fi
}

assert_nz() {
    if [ -z "$1" ]; then err "assert_nz $2"; fi
}

# Run a command that should never fail. If the command fails execution
# will immediately terminate with an error showing the failing
# command.
ensure() {
    "$@"
    need_ok "command failed: $*"
}

# This is just for indicating that commands' results are being
# intentionally ignored. Usually, because it's being executed
# as part of error handling.
ignore() {
    run "$@"
}

# Runs a command and prints it to stderr if it fails.
run() {
    "$@"
    local _retval=$?
    if [ $_retval != 0 ]; then
	say_err "command failed: $*"
    fi
    return $_retval
}

# Prints the absolute path of a directory to stdout
abs_path() {
    local _path="$1"
    # Unset CDPATH because it causes havok: it makes the destination unpredictable
    # and triggers 'cd' to print the path to stdout. Route `cd`'s output to /dev/null
    # for good measure.
    (unset CDPATH && cd "$_path" > /dev/null && pwd)
}

assert_cmds() {
    need_cmd dirname
    need_cmd basename
    need_cmd mkdir
    need_cmd cat
    need_cmd curl
    need_cmd mktemp
    need_cmd rm
    need_cmd egrep
    need_cmd grep
    need_cmd file
    need_cmd uname
    need_cmd tar
    need_cmd sed
    need_cmd sh
    need_cmd mv
    need_cmd awk
    need_cmd cut
    need_cmd sort
    need_cmd date
    need_cmd head
    need_cmd printf
    need_cmd touch
    need_cmd id
}

main "$@"
