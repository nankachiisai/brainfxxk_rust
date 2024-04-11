use std::fs::File;
use std::io::Read;
use std::env;

mod interpreter;
use interpreter::Interpreter;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = get_command_line_arguments();
    if args.len() != 2 {
        println!("missing file name!");
        println!("Usage: {} <file name>", args[0]);
        return;
    }
    let filename = args[1].as_str();

    // ファイルを読み込む
    let code = match read_file(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error reading file: {}", e),
    };

    // 実行する
    let mut ipreter = Interpreter::new(code.as_str(), "test");
    match ipreter.run() {
        Ok(_output) => println!("{}", _output),
        Err(e_run) => panic!("Interpreter Error: {}", e_run),
    }
}

// 与えられたパスのファイルを開き、その内容を文字列として返す関数
fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?; // ファイルを開く

    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // ファイルの内容を文字列として読み込む

    Ok(contents)
}

// コマンドライン引数を取得する関数
fn get_command_line_arguments() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args.to_vec()
}
