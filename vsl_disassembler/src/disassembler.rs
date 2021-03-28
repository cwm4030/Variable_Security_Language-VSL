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

const F_CONSTANT: i64 = 15;
const F_ADD: i64 = 16;
const F_SUB: i64 = 17;
const F_MUL: i64 = 18;
const F_DIV: i64 = 19;
const F_EQUAL: i64 = 20;
const F_LESS: i64 = 21;
const F_GREATER: i64 = 22;
const F_NOT_EQUAL: i64 = 23;
const F_LESS_EQUAL: i64 = 24;
const F_GREATER_EQUAL: i64 = 25;

const S_CONSTANT: i64 = 26;
const S_ADD: i64 = 27;
const S_EQUAL: i64 = 28;
const S_NOT_EQUAL: i64 = 29;

const OP_AND: i64 = 30;
const OP_OR: i64 = 31;

const JUMP_IF_FALSE: i64 = 32;
const JUMP: i64 = 33;

const CALL: i64 = 34;
const RETURN_VAL: i64 = 35;
const RETURN_NON_VAL: i64 = 36;
const ARG_LOAD: i64 = 37;
const ARG_STORE: i64 = 38;

const USE: i64 = 39;

const HALT: i64 = 40;

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
                S_EQUAL => println!("{}: {}", self.ip - 1, "s_equal"),
                S_NOT_EQUAL => println!("{}: {}", self.ip - 1, "s_not_equal"),
                OP_AND => println!("{}: {}", self.ip - 1, "op_and"),
                OP_OR => println!("{}: {}", self.ip - 1, "op_or"),
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
                USE => {
                    if self.code[self.ip] == 0 {
                        println!("{}: {}", self.ip - 1, "use print");
                        self.ip += 2;
                    } else if self.code[self.ip] == 1 {
                        println!("{}: {}", self.ip - 1, "use read");
                        self.ip += 2;
                    } else if self.code[self.ip] == 2 {
                        println!("{}: {}", self.ip - 1, "use string_to_int");
                        self.ip += 1;
                    } else if self.code[self.ip] == 3 {
                        println!("{}: {}", self.ip - 1, "use string_to_float");
                        self.ip += 1;
                    } else if self.code[self.ip] == 4 {
                        println!("{}: {}", self.ip - 1, "use int_to_float");
                        self.ip += 1;
                    } else if self.code[self.ip] == 5 {
                        println!("{}: {}", self.ip - 1, "use int_to_string");
                        self.ip += 1;
                    } else if self.code[self.ip] == 6 {
                        println!("{}: {}", self.ip - 1, "use float_to_int");
                        self.ip += 1;
                    } else if self.code[self.ip] == 7 {
                        println!("{}: {}", self.ip - 1, "use float_to_string");
                        self.ip += 1;
                    } else if self.code[self.ip] == 8 {
                        println!("{}: {}", self.ip - 1, "use get_string_index");
                        self.ip += 1;
                    } else if self.code[self.ip] == 9 {
                        println!("{}: {}", self.ip - 1, "use set_string_index");
                        self.ip += 1;
                    } else if self.code[self.ip] == 10 {
                        println!("{}: {}", self.ip - 1, "use get_copy_string");
                        self.ip += 1;
                    }
                }
                HALT => println!("{}: {}", self.ip - 1, "halt"),
                _ => panic!("Bad Opcode: {}", opcode),
            }
        }
    }
}