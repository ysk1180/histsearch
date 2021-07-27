use std::io;

pub fn input() -> String {
    println!("Input word for search!");

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read word");

    // 入力で入ってくる改行コードを削除
    let word = word.trim();

    word.to_string()
}
