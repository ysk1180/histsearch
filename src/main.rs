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

    // readerでエラーしているもの（おそらく日本語などのASCIIコード以外が入っていたもの）を排除する
    let mut commands: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    commands.reverse(); // 最近打ったコマンドを上位に表示させたいので新しい順にしている

    // TODO: 内容がない行の削除
    // TODO: 先頭の不要な文字の削除

    let mut i = 0;
    for line in commands.iter() {
        println!("{}", line);
        i += 1;
        if i == 100 {
            break;
        }
    }

    // let mut commands: Vec<_> = reader
    //     .lines()
    //     .map(|line| {
    //         let line = line.unwrap();
    //         let splited: Vec<&str> = line.split(":0;").collect();
    //         splited.get(1).unwrap();
    //     })
    //     .collect();
    // commands.reverse();

    // for line in reader.lines() {
    //     let line = line.unwrap();
    //     let vec: Vec<&str> = line.split(":0;").collect();
    //     let command = vec.get(1).unwrap();
    //     println!("{}", command);

    //     i += 1;
    //     if i == 10 {
    //         break;
    //     }
    // }
}

fn find_history_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".zhistory");
        path
    })
}
