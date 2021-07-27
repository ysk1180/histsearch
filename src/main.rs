use std::{fs::OpenOptions, io::BufReader};

use input_commandline::input;
use search::filter;
use simplified_command::simplify;

mod display;
mod history;
mod input_commandline;
mod search;
mod simplified_command;
use crate::{display::display_commandline, history::find_history_file};

fn main() {
    let input_word = input();

    let file_path = find_history_file().expect("can't find history file.");

    let file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("can't open history file.");

    let reader = BufReader::new(file);

    let commands = simplify(reader);

    let commands = filter(commands, input_word);

    display_commandline(commands);
}
