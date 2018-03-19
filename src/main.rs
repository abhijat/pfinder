extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate users;

use process::Process;
use search::Search;
use std::io::{Error, ErrorKind, Read};
use utils::is_numeric;
use utils::make_error;
use utils::remove_nulls;

mod process;
mod search;
mod utils;


fn get_user_name(path: &mut std::path::PathBuf) -> std::io::Result<String> {
    path.push("loginuid");
    if !path.exists() {
        return Err(Error::new(ErrorKind::NotFound, format!("missing path: {:?}", path)));
    }

    let mut file = std::fs::File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let uid = buffer.parse::<u32>();
    if uid.is_err() {
        return make_error("unable to parse uid");
    }

    let uid = uid.unwrap();
    match users::get_user_by_uid(uid) {
        Some(user) => Ok(user.name().to_owned()),
        None => make_error(&format!("no user for id {}", uid)),
    }
}

fn get_command_string(path: &mut std::path::PathBuf) -> std::io::Result<String> {
    path.push("cmdline");
    if !path.exists() {
        return Err(Error::new(ErrorKind::NotFound, format!("missing path: {:?}", path)));
    }

    let mut file = std::fs::File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    Ok(remove_nulls(&buffer))
}

fn parse_args() -> Search {
    let app = clap::App::new("pfind");
    let exclude_arg = clap::Arg::with_name("exclude")
        .short("x")
        .long("exclude")
        .help("exclude this term from search");

    let case_sensitive_arg = clap::Arg::with_name("case_sensitive_off")
        .short("i")
        .help("turn off case sensitive matches");

    let term_arg = clap::Arg::with_name("search_term")
        .required(true);

    let matches = app.arg(term_arg)
        .arg(exclude_arg)
        .arg(case_sensitive_arg)
        .get_matches();

    let term = matches.value_of("search_term").unwrap();
    let invert_search = matches.is_present("exclude");
    let case_sensitive = !matches.is_present("case_sensitive_off");

    Search::new(invert_search, case_sensitive, term)
}

fn main() {
    let search = parse_args();

    let dir = std::fs::read_dir("/proc")
        .expect("unable to read /proc");

    let mut processes: Vec<Process> = Vec::new();
    for item in dir {
        if let Ok(dir_entry) = item {
            let path_name = dir_entry.file_name().into_string();
            if let Ok(path_as_string) = path_name {
                if !is_numeric(&path_as_string) {
                    continue;
                }

                let cmd_result = get_command_string(&mut dir_entry.path());
                let user_result = get_user_name(&mut dir_entry.path());

                if cmd_result.is_ok() && user_result.is_ok() {
                    let cmdline = cmd_result.unwrap();
                    if !cmdline.is_empty() {
                        processes.push(Process::new(&user_result.unwrap(), &cmdline));
                    }
                }
            }
        }
    }

    for process in &processes {
        if process.matches(&search) {
            println!("{}", serde_json::to_string_pretty(&process).unwrap());
        }
    }
}