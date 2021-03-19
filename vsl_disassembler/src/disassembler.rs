const POP: i64 = 1;
const LOCAL_LOAD: i64 = 2;
const LOCAL_STORE: i64 = 3;

const I_CONSTANT: i64 = 4;
const I_ADD: i64 = 5;
const I_SUB: i64 = 6;
const I_MUL: i64 = 7;
const I_DIV: i64 = 8;
const I_EQUAL: i64 = 9;
const I_LESS: i64 = 10;
const I_GREATER: i64 = 11;
const I_NOT_EQUAL: i64 = 12;
const I_LESS_EQUAL: i64 = 13;
const I_GREATER_EQUAL: i64 = 14;
const I_AND: i64 = 15;
const I_OR: i64 = 16;

const F_CONSTANT: i64 = 17;
const F_ADD: i64 = 18;
const F_SUB: i64 = 19;
const F_MUL: i64 = 20;
const F_DIV: i64 = 21;
const F_EQUAL: i64 = 22;
const F_LESS: i64 = 23;
const F_GREATER: i64 = 24;
const F_NOT_EQUAL: i64 = 25;
const F_LESS_EQUAL: i64 = 26;
const F_GREATER_EQUAL: i64 = 27;
const F_AND: i64 = 28;
const F_OR: i64 = 29;

const S_CONSTANT: i64 = 30;
const S_ADD: i64 = 31;
const S_LOAD: i64 = 32;
const S_STORE: i64 = 33;
//const S_JUMP_EQUAL: i64 = 34;
//const S_JUMP_NOT_EQUAL: i64 = 35;

const JUMP_IF_FALSE: i64 = 36;
const JUMP: i64 = 37;

const CALL: i64 = 38;
const RETURN_VAL: i64 = 39;
const RETURN_NON_VAL: i64 = 40;
const ARG_LOAD: i64 = 41;
const ARG_STORE: i64 = 42;

const HALT: i64 = 43;
const I_PRINT: i64 = 44;
const F_PRINT: i64 = 45;
const S_PRINT: i64 = 46;

pub struct Disassembler {
    ip: usize,
    code: Vec<i64>,
}

impl Disassembler {
    pub fn new(program: Vec<i64>) -> Disassembler {
        let mut disassembler = Disassembler {
            ip: 0,
            code: Vec::new(),
        };
        
        for chunck in program {
            disassembler.code.push(chunck);
        }
        disassembler
    }

    pub fn disassemble(&mut self) {
        let length: usize = self.code.len();

        while self.ip < length {
            let opcode = self.code[self.ip];
            self.ip += 1;

            match opcode {
                POP => {
                    println!("{}: {} {}", self.ip - 1, "pop", self.code[self.ip]);
                    self.ip += 1;
                },
                LOCAL_LOAD => {
                    println!("{}: {} {}", self.ip - 1, "local_load", self.code[self.ip]);
                    self.ip += 1;
                },
                LOCAL_STORE => {
                    println!("{}: {} {}", self.ip - 1, "local_store", self.code[self.ip]);
                    self.ip += 1;
                },
                I_CONSTANT => {
                    println!("{}: {} {}", self.ip - 1, "i_constant", self.code[self.ip]);
                    self.ip += 1;
                },
                I_ADD => println!("{}: {}", self.ip - 1, "i_add"),
                I_SUB => println!("{}: {}", self.ip - 1, "i_sub"),
                I_MUL => println!("{}: {}", self.ip - 1, "i_mul"),
                I_DIV => println!("{}: {}", self.ip - 1, "i_div"),
                I_EQUAL => println!("{}: {}", self.ip - 1, "i_equal"),
                I_LESS => println!("{}: {}", self.ip - 1, "i_less"),
                I_GREATER => println!("{}: {}", self.ip - 1, "i_greater"),
                I_NOT_EQUAL => println!("{}: {}", self.ip - 1, "i_not_equal"),
                I_LESS_EQUAL => println!("{}: {}", self.ip - 1, "i_less_equal"),
                I_GREATER_EQUAL => println!("{}: {}", self.ip - 1, "i_greater_equal"),
                I_OR => println!("{}: {}", self.ip - 1, "i_or"),
                I_AND => println!("{}: {}", self.ip - 1, "i_and"),
                F_CONSTANT => {
                    println!("{}: {} {}", self.ip - 1, "f_constant", f64::from_be_bytes(self.code[self.ip].to_be_bytes()));
                    self.ip += 1;
                },
                F_ADD => println!("{}: {}", self.ip - 1, "f_add"),
                F_SUB => println!("{}: {}", self.ip - 1, "f_sub"),
                F_MUL => println!("{}: {}", self.ip - 1, "f_mul"),
                F_DIV => println!("{}: {}", self.ip - 1, "f_div"),
                F_EQUAL => println!("{}: {}", self.ip - 1, "f_equal"),
                F_LESS => println!("{}: {}", self.ip - 1, "f_less"),
                F_GREATER => println!("{}: {}", self.ip - 1, "f_greater"),
                F_NOT_EQUAL => println!("{}: {}", self.ip - 1, "f_not_equal"),
                F_LESS_EQUAL => println!("{}: {}", self.ip - 1, "f_less_equal"),
                F_GREATER_EQUAL => println!("{}: {}", self.ip - 1, "f_greater_equal"),
                F_OR => println!("{}: {}", self.ip - 1, "f_or"),
                F_AND => println!("{}: {}", self.ip - 1, "f_and"),
                S_CONSTANT => {
                    let index = self.ip - 1;
                    let mut string: String = String::new();
                    while self.code[self.ip] != 0 && self.code[self.ip] < 128 {
                        if self.code[self.ip] != 10 {
                            string.push(self.code[self.ip] as u8 as char);
                        } else {
                            string.push('\\');
                            string.push('n');
                        }
                        self.ip += 1;
                    }
                    println!("{}: {} \"{}\"", index, "s_constant", string);
                    self.ip += 1;
                },
                S_ADD => println!("{}: {}", self.ip - 1, "s_add"),
                S_LOAD => {
                    println!("{}: {} {}", self.ip - 1, "s_load", self.code[self.ip]);
                    self.ip += 1;
                },
                S_STORE => {
                    println!("{}: {} {}", self.ip - 1, "s_store", self.code[self.ip]);
                    self.ip += 1;
                },
                JUMP_IF_FALSE => {
                    println!("{}: {} {}", self.ip - 1, "jump_if_false", self.code[self.ip]);
                    self.ip += 1;
                },
                JUMP => {
                    println!("{}: {} {}", self.ip - 1, "jump", self.code[self.ip]);
                    self.ip += 1;
                },
                CALL => {
                    println!("{}: {} {} {}", self.ip - 1, "call", self.code[self.ip], self.code[self.ip + 1]);
                    self.ip += 2;
                },
                RETURN_VAL => println!("{}: {}", self.ip - 1, "return_val"),
                RETURN_NON_VAL => println!("{}: {}", self.ip - 1, "return_non_val"),
                ARG_LOAD => {
                    println!("{}: {} {}", self.ip - 1, "arg_load", self.code[self.ip]);
                    self.ip += 1;
                },
                ARG_STORE => {
                    println!("{}: {} {}", self.ip - 1, "arg_store", self.code[self.ip]);
                    self.ip += 1;
                },
                HALT => println!("{}: {}", self.ip - 1, "halt"),
                I_PRINT => println!("{}: {}", self.ip - 1, "i_print"),
                F_PRINT => println!("{}: {}", self.ip - 1, "f_print"),
                S_PRINT => println!("{}: {}", self.ip - 1, "s_print"),
                _ => panic!("Bad Opcode: {}", opcode),
            }
        }
    }
}