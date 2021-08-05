use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "histsearch",
    about = "Simple history search tool"
)]
pub struct CommandLineArgs {
    /// history file name
    #[structopt(parse(from_os_str), short, long)]
    pub history_file: Option<PathBuf>
}
