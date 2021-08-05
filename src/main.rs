use std::{fs::OpenOptions, io::BufReader, path::PathBuf};

use cli::CommandLineArgs;
use input_commandline::input;
use search::filter;
use simplified_command::simplify;
use structopt::StructOpt;

mod display;
mod history;
mod input_commandline;
mod search;
mod simplified_command;
mod cli;
use crate::{display::display_commandline, history::find_history_file};

fn main() {
    let input_word = input();

    let CommandLineArgs { history_file } = cli::CommandLineArgs::from_args();

    let history_file = history_file.unwrap_or(PathBuf::from(".zsh_history")); // .zhistory
    let file_path = find_history_file(history_file).expect("can't find history file.");

    let file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("can't open history file.");

    let reader = BufReader::new(file);

    let commands = simplify(reader);

    let commands = filter(commands, input_word);

    display_commandline(commands);
}
