use std::path::PathBuf;

pub fn find_history_file() -> Option<PathBuf> {
    // let history_file_path = ".zhistory";
    let history_file_path = ".zsh_history";
    home::home_dir().map(|mut path| {
        path.push(history_file_path);
        path
    })
}
