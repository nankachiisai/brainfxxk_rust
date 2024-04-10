use core::fmt;
use std::error;
use std::fs::File;
use std::io::Read;
use std::env;

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
    match run(&code, "test") {
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

// インタープリタ実行時のエラーを表す構造体
#[derive(Debug, Clone)]
struct InterpretError {
    message: String,
}

impl InterpretError {
    fn new(message: &str) -> InterpretError {
        InterpretError {
            message: message.to_string(),
        }
    }
}

impl error::Error for InterpretError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

// brainfxxkコードを実行する関数
fn run(code: &str, input: &str) -> Result<String, InterpretError> {
    // input pointer
    let mut inp = 0;

    // input memory
    let input: Vec<u8> = Vec::from(input.as_bytes());

    // output pointer
    let mut oup = 0;

    // output memory
    let mut output: Vec<u8> = vec![0; 1000];

    // instruction pointer
    let mut ip = 0;

    // instruction memory
    let i_memory: Vec<u8> = Vec::from(code.as_bytes());

    // data pointer
    let mut dp = 0;

    // data memory
    let mut d_memory: Vec<u8> = vec![0; 1000];

    loop {
        // for debug
        // println!("i_memory[{}]: {}", ip, i_memory[ip] as char);
        // println!("d_memory[{}]: {}", dp, d_memory[dp]);
        // println!("");

        match i_memory[ip] {
            b'>' => dp += 1,                 // increment pointer
            b'<' => dp -= 1,                 // decrement pointer
            b'+' => d_memory[dp] += 1,       // increment value
            b'-' => d_memory[dp] -= 1,       // decrement value
            b'.' => {                        // output value
                output[oup] = d_memory[dp];
                oup += 1;
            },
            b',' => {                        // input value
                d_memory[dp] = input[inp];
                inp += 1;
            },
            b'[' => {                        // jump forward
                if d_memory[dp] == 0 {
                    while i_memory[ip] != b']' {
                        ip += 1;
                    }
                    ip += 1;
                }
            },
            b']' => {                        // jump backward
                if d_memory[dp] != 0 {
                    while i_memory[ip] != b'[' {
                        ip -= 1;
                    }
                    ip -= 1;
                }
            },
            _ => (),
        }

        if i_memory.len() - 1 == ip {
            break;
        } else {
            ip += 1;
        }
    }

    let ret = String::from_utf8(output).unwrap();
    Ok(ret)
}