use crate::{command_prelude::*, CliResult};
use clap::{ArgMatches, Command};

pub type Exec = fn(&mut GlobalContext, &ArgMatches) -> CliResult;

pub fn cli() -> Vec<Command> {
    vec![
        init::cli(),
        add::cli(),
        list::cli(),
        update::cli(),
        delete::cli(),
        subcategory::cli(),
        total::cli(),
    ]
}

pub fn build_exec(cmd: &str) -> Option<Exec> {
    match cmd {
        "init" => Some(init::exec),
        "add" => Some(add::exec),
        "list" => Some(list::exec),
        "update" => Some(update::exec),
        "delete" => Some(delete::exec),
        "subcategory" => Some(subcategory::exec),
        "total" => Some(total::exec),
        _ => None,
    }
}

pub mod add;
pub mod delete;
pub mod init;
pub mod list;
pub mod subcategory;
pub mod total;
pub mod update;
