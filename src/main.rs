use std::{
    fs::OpenOptions,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

fn main() {
    println!("Input word for search!");

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read word");

    println!("{}", word);

    let file_path = find_history_file().expect("can't find history file.");

    let file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("can't open history file.");

    let reader = BufReader::new(file);

    let mut i = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);

        i += 1;
        if i == 10 {
            break;
        }
    }
}

fn find_history_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".zhistory");
        path
    })
}
