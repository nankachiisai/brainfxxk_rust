use core::fmt;
use std::error;

#[derive(Debug, Clone)]
pub struct Interpreter {
    p_input: usize,    // input pointer
    m_input: Vec<u8>,  // input memory
    p_output: usize,   // output pointer
    m_output: Vec<u8>, // output memory
    p_inst: usize,     // instruction pointer
    m_inst: Vec<u8>,   // instruction memory
    p_data: usize,     // data pointer
    m_data: Vec<u8>,   // data memory
}

// 実行成功時の戻り値
pub enum Success {
    Running, // プログラムが実行途中であることを表す
    Exit,    // プログラムを実行し終わったことを表す
}

impl Interpreter {
    // 構造体を初期化する
    pub fn new(instruction: &str, input: &str) -> Interpreter {
        Interpreter {
            p_input: 0,
            m_input: Vec::from(input.as_bytes()),
            p_output: 0,
            m_output: vec![0; 1000],
            p_inst: 0,
            m_inst: Vec::from(instruction.as_bytes()),
            p_data: 0,
            m_data: vec![0; 1000],
        }
    }

    // brainfxxkコードを与えられた入力に従って実行する
    pub fn run(&mut self) -> Result<String, InterpretError> {
        // プログラムを実行し終わるまでループ
        loop {
            match self.step() {
                Ok(Success::Running) => (),
                Ok(Success::Exit) => {
                    return Ok(String::from_utf8(self.m_output.clone()).unwrap());
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    // 1ステップごとに実行する
    pub fn step(&mut self) -> Result<Success, InterpretError> {
        match self.m_inst[self.p_inst] {
            b'>' => self.p_data += 1,              // increment pointer
            b'<' => self.p_data -= 1,              // decrement pointer
            b'+' => self.m_data[self.p_data] += 1, // increment value
            b'-' => self.m_data[self.p_data] -= 1, // decrement value
            b'.' => {
                // output value
                self.m_output[self.p_output] = self.m_data[self.p_data];
                self.p_output += 1;
            }
            b',' => {
                // input value
                self.m_data[self.p_data] = self.m_input[self.p_input];
                self.p_input += 1;
            }
            b'[' => {
                // jump forward
                if self.m_data[self.p_data] == 0 {
                    while self.m_inst[self.p_inst] != b']' {
                        self.p_inst += 1;
                    }
                    self.p_inst += 1;
                }
            }
            b']' => {
                // jump backward
                if self.m_data[self.p_data] != 0 {
                    while self.m_inst[self.p_inst] != b'[' {
                        self.p_inst -= 1;
                    }
                    self.p_inst -= 1;
                }
            }
            _ => (),
        }

        // インストラクションポインタを進める
        // 最後まで実行したら終了
        if self.m_inst.len() - 1 == self.p_inst {
            Ok(Success::Exit)
        } else {
            self.p_inst += 1;
            Ok(Success::Running)
        }
    }
}

// インタープリタ実行時のエラーを表す構造体
#[derive(Debug, Clone)]
pub struct InterpretError {
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
