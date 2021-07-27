use std::{
    fs::OpenOptions,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use regex::Regex;

fn main() {
    println!("Input word for search!");

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read word");

    // 入力で入ってくる改行コードを削除
    let word = word.trim();

    let file_path = find_history_file().expect("can't find history file.");

    let file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("can't open history file.");

    let reader = BufReader::new(file);

    // readerでエラーしているもの（おそらく日本語などのASCIIコード以外が入っていたもの）を排除する
    let commands: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    // 空行や複数行コマンドの2行目以降などの不要なデータを排除する
    let re = Regex::new(r":0;").unwrap();
    let mut commands: Vec<String> = commands
        .iter()
        .filter(|command| re.is_match(command))
        .map(|c| c.to_string())
        .collect();

    commands.reverse(); // 最近打ったコマンドを上位に表示させたいので新しい順にしている

    // 「: 1627024977:0;ts」みたいな形式で入っているのでコマンドだけを抽出する
    let re = Regex::new(r"^.+:0;").unwrap();
    let commands: Vec<String> = commands
        .iter()
        .map(|c| re.replace(c, ""))
        .map(|c| c.to_string())
        .collect();

    // 入力された文字列とコマンドの先頭部分が一致するコマンドを一覧できるようにする
    let commands: Vec<String> = commands
        .iter()
        .filter(|c| {
            let word_count = word.chars().count();
            let c_count = c.chars().count();

            if word_count > c_count {
                return false;
            }

            let sliced = &c[..word_count];
            sliced.to_string() == word
        })
        .map(|c| c.to_string())
        .collect();

    // 該当したコマンドを100件まで表示する
    let mut i = 0;
    for line in commands.iter() {
        println!("{}", line);
        i += 1;
        if i == 10 {
            break;
        }
    }
}

fn find_history_file() -> Option<PathBuf> {
    let history_file_path = ".zhistory";
    // let history_file_path = ".zsh_history";
    home::home_dir().map(|mut path| {
        path.push(history_file_path);
        path
    })
}
