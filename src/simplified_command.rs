use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

pub fn simplify(reader: BufReader<File>) -> Vec<String> {
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

    // 重複排除（参考にしたサイト:https://qiita.com/yagince/items/73184237964e9dbb8b3d ）
    let commands: HashSet<String> = commands.into_iter().collect();

    let commands: Vec<String> = commands.into_iter().collect();

    commands
}
