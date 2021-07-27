pub fn filter(commands: Vec<String>, word: String) -> Vec<String> {
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

    commands
}
