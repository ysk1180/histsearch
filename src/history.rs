use std::path::PathBuf;

pub fn find_history_file(file: PathBuf) -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(file);
        path
    })
}
