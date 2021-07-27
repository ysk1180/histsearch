pub fn display_commandline(commands: Vec<String>) {
    println!("----------");

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
