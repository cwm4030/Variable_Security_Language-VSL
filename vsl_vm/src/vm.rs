use std::io::Write;
use std::io;

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

//----------------------------------------------------------------------------------------------------

// data types
const INT: i64 = 0;
const FLOAT: i64 = 1;
const STRING: i64 = 2;

// functions
const PRINT: i64 = 0;
const READ: i64 = 1;
const STRING_TO_INT: i64 = 2;
const STRING_TO_FLOAT: i64 = 3;
const INT_TO_FLOAT: i64 = 4;
const INT_TO_STRING: i64 = 5;
const FLOAT_TO_INT: i64 = 6;
const FLOAT_TO_STRING: i64 = 7;
const GET_STRING_INDEX: i64 = 8;
const SET_STRING_INDEX: i64 = 9;
const GET_COPY_STRING: i64 = 10;


 //---------------------------------------------------------------------------------------------------

pub struct VM {
    string_constants: Vec<String>,
    int_vec_constants: Vec<Vec<i64>>,
    float_vec_constants: Vec<Vec<f64>>,
    string_vec_constants: Vec<Vec<String>>,
    stack: Vec<i64>,
    code: Vec<i64>,
    ip: usize,
    fp: usize,
    sp: usize,
    debug: bool,
    pub halt: bool,
}

impl VM {
    pub fn new(program: Vec<i64>, debug: bool) -> VM {
        let mut vm = VM {
            string_constants: Vec::new(),
            int_vec_constants: Vec::new(),
            float_vec_constants: Vec::new(),
            string_vec_constants: Vec::new(),
            stack: Vec::new(),
            code: Vec::new(),
            ip: 0,
            fp: 0,
            sp: 0,
            debug: debug,
            halt: false,
        };
        
        for chunck in program {
            vm.code.push(chunck);
        }
        vm
    }

    pub fn execute(&mut self) {
        while self.halt == false {
            let opcode = self.code[self.ip];
            self.ip += 1;

            match opcode {
                POP => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "pop", self.code[self.ip]);
                    }
                    let index = self.code[self.ip] as usize + self.fp + 3;
                    self.stack.remove(index);
                    self.sp -= 1;
                    self.ip += 1;
                },
                LOCAL_LOAD => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "local_load", self.code[self.ip]);
                    }
                    let index: usize = self.code[self.ip] as usize + self.fp + 3;
                    self.stack.push(self.stack[index]);
                    self.ip += 1;
                    self.sp += 1;
                },
                LOCAL_STORE => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "local_store", self.code[self.ip]);
                    }
                    let index: usize = self.code[self.ip] as usize + self.fp + 3;
                    let data = self.stack[self.sp - 1];
                    if index < self.stack.len() - 1 {
                        self.stack[index] = data;
                        self.stack.pop();
                        self.sp -= 1;
                    }
                    self.ip += 1;
                },
                I_CONSTANT => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "i_constant", self.code[self.ip]);
                    }
                    self.stack.push(self.code[self.ip]);
                    self.ip += 1;
                    self.sp += 1;
                },
                I_ADD => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_add");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    let (result, _did_overflow) = a.overflowing_add(b);
                    self.stack.push(result);
                    self.sp -= 1;
                },
                I_SUB => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_sub");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    let (result, _did_overflow) = a.overflowing_sub(b);
                    self.stack.push(result);
                    self.sp -= 1;
                },
                I_MUL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_mul");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    let (result, _did_overflow) = a.overflowing_mul(b);
                    self.stack.push(result);
                    self.sp -= 1;
                },
                I_DIV => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_div");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    let (result, _did_overflow) = a.overflowing_div(b);
                    self.stack.push(result);
                    self.sp -= 1;
                },
                I_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_equal");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a == b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                I_LESS => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_less");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a < b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                I_GREATER => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_greater");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a > b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                I_NOT_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_not_equal");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a != b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                I_LESS_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_less_equal");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a <= b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                I_GREATER_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_greater_equal");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a >= b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                F_CONSTANT => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "f_constant", f64::from_be_bytes(self.code[self.ip].to_be_bytes()));
                    }
                    self.stack.push(self.code[self.ip]);
                    self.ip += 1;
                    self.sp += 1;
                },
                F_ADD => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_add");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    let answer_bytes = (a + b).to_be_bytes();
                    let answer = i64::from_be_bytes(answer_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    self.stack.push(answer);
                    self.sp -= 1;
                },
                F_SUB => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_sub");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    let answer_bytes = (a - b).to_be_bytes();
                    let answer = i64::from_be_bytes(answer_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    self.stack.push(answer);
                    self.sp -= 1;
                },
                F_MUL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_mul");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    let answer_bytes = (a * b).to_be_bytes();
                    let answer = i64::from_be_bytes(answer_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    self.stack.push(answer);
                    self.sp -= 1;
                },
                F_DIV => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_div");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    let answer_bytes = (a / b).to_be_bytes();
                    let answer = i64::from_be_bytes(answer_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    self.stack.push(answer);
                    self.sp -= 1;
                },
                F_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_equal");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a == b {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                F_LESS => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_less");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a < b {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                F_GREATER => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_greater");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a > b {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                F_NOT_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_not_equal");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a != b {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                F_LESS_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_less_equal");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a <= b {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                F_GREATER_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "f_greater_equal");
                    }
                    let a_bytes = self.stack[self.sp - 2].to_be_bytes();
                    let b_bytes = self.stack[self.sp - 1].to_be_bytes();
                    let a = f64::from_be_bytes(a_bytes);
                    let b = f64::from_be_bytes(b_bytes);
                    self.stack.pop();
                    self.stack.pop();
                    let float_true: f64 = 1.0;
                    if a >= b {
                        self.stack.push(i64::from_be_bytes(float_true.to_be_bytes()));
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                S_CONSTANT => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "s_constant");
                    }
                    let mut string: String = String::new();
                    while self.code[self.ip] != 0 && self.code[self.ip] < 128 {
                        string.push(self.code[self.ip] as u8 as char);
                        self.ip += 1;
                    }
                    self.string_constants.push(string);
                    self.ip += 1;

                    self.stack.push(self.string_constants.len() as i64 - 1);
                    self.sp += 1;
                },
                S_ADD => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "s_add");
                    }
                    let a = self.stack[self.sp - 2] as usize;
                    let b = self.stack[self.sp - 1] as usize;
                    self.stack.pop();
                    self.stack.pop();

                    let new_string = self.string_constants[a].clone() + &self.string_constants[b].clone();
                    self.string_constants.push(new_string);
                    self.stack.push(self.string_constants.len() as i64 - 1);
                    self.sp -= 1;
                },
                S_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "s_equal");
                    }
                    let a = self.stack[self.sp - 2] as usize;
                    let b = self.stack[self.sp - 1] as usize;
                    self.stack.pop();
                    self.stack.pop();
                    if self.string_constants[a] == self.string_constants[b] {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                S_NOT_EQUAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "s_not_equal");
                    }
                    let a = self.stack[self.sp - 2] as usize;
                    let b = self.stack[self.sp - 1] as usize;
                    self.stack.pop();
                    self.stack.pop();
                    if self.string_constants[a] != self.string_constants[b] {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                OP_AND => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_and");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a != 0 && b != 0 {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                OP_OR => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "i_or");
                    }
                    let a = self.stack[self.sp - 2];
                    let b = self.stack[self.sp - 1];
                    self.stack.pop();
                    self.stack.pop();
                    if a != 0 || b != 0 {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                    self.sp -= 1;
                },
                JUMP_IF_FALSE => {
                    if self.debug {
                        println!("{}: {} {} {}", self.ip - 1, "jump_if_false", self.stack[self.sp - 1], self.code[self.ip]);
                    }
                    let location = self.code[self.ip] as usize;
                    let boolean_value = self.stack[self.sp - 1];
                    self.stack.pop();
                    if boolean_value == 0 {
                        self.ip = location;
                    } else {
                        self.ip += 1;
                    }
                    self.sp -= 1;
                },
                JUMP => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "jump", self.code[self.ip]);
                    }
                    let location = self.code[self.ip] as usize;
                    self.ip = location;
                },
                CALL => {
                    if self.debug {
                        println!("{}: {} {} {}", self.ip - 1, "call", self.code[self.ip], self.code[self.ip + 1]);
                    }
                    let address: i64 = self.code[self.ip];
                    let nargs: i64 = self.code[self.ip + 1];
                    let return_address: i64 = self.ip as i64 + 2;

                    self.stack.push(return_address);
                    self.stack.push(nargs);

                    self.stack.push(self.fp as i64);
                    self.fp = self.sp;

                    self.sp += 3;
                    self.ip = address as usize;
                },
                RETURN_VAL => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "return_val", self.stack[self.sp - 1]);
                    }
                    let return_value: i64 = self.stack[self.sp - 1];
                    let fp: usize = self.stack[self.sp - 2] as usize;
                    let nargs: usize = self.stack[self.sp - 3] as usize;
                    let return_address: usize = self.stack[self.sp - 4] as usize;
                    self.sp = self.fp - nargs;
                    self.fp = fp;
                    self.ip = return_address;

                    while self.sp != self.stack.len() {
                        self.stack.pop();
                    }
                    self.stack.push(return_value);
                    self.sp += 1;
                },
                RETURN_NON_VAL => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "return_non_val");
                    }
                    let fp: usize = self.stack[self.sp - 1] as usize;
                    let nargs: usize = self.stack[self.sp - 2] as usize;
                    let return_address: usize = self.stack[self.sp - 3] as usize;
                    self.sp = self.fp - nargs;
                    self.fp = fp;
                    self.ip = return_address;

                    while self.sp != self.stack.len() {
                        self.stack.pop();
                    }
                },
                ARG_LOAD => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "arg_load", self.code[self.ip]);
                    }
                    let offset: i64 = self.code[self.ip];
                    let num_args: i64 = self.stack[self.fp + 1];
                    let value: i64 = self.stack[self.fp - num_args as usize + offset as usize];
                    self.stack.push(value);
                    self.sp += 1;
                    self.ip += 1;
                },
                ARG_STORE => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "arg_store", self.code[self.ip]);
                    }
                    let offset: i64 = self.code[self.ip];
                    let num_args: i64 = self.stack[self.fp + 1];
                    let value: i64 = self.stack[self.sp - 1];
                    self.stack[self.fp - num_args as usize + offset as usize] = value;
                    self.stack.pop();
                    self.sp -= 1;
                    self.ip += 1;
                },
                USE => {
                    if self.debug {
                        println!("{}: {} {}", self.ip - 1, "use", self.code[self.ip]);
                    }
                    self.standard_library();
                },
                HALT => {
                    if self.debug {
                        println!("{}: {}", self.ip - 1, "halt");
                    }
                    self.halt = true;
                },
                _ => panic!("Bad Opcode: {}", opcode),
            }
            if self.debug {
                println!("{:?} {} {}", self.stack, self.sp, self.fp);
                println!("{:?}\n", self.string_constants);
            }
        }
    }

    fn standard_library(&mut self) {
        match self.code[self.ip] {
            PRINT => {
                self.ip += 1;
                if self.code[self.ip] == INT {
                    self.ip += 1;
                    print!("{}", self.stack[self.sp - 1]);
                    std::io::stdout().flush().expect("Failed to flush stdout.");
                    self.stack.pop();
                    self.sp -= 1;
                } else if self.code[self.ip] == FLOAT {
                    self.ip += 1;
                    print!("{}", f64::from_be_bytes(self.stack[self.sp - 1].to_be_bytes()));
                    self.stack.pop();
                    self.sp -= 1;
                } else if self.code[self.ip] == STRING {
                    self.ip += 1;
                    print!("{}", self.string_constants[self.stack[self.sp - 1] as usize]);
                    std::io::stdout().flush().expect("Failed to flush stdout.");
                    self.stack.pop();
                    self.sp -= 1;
                }
            },
            READ => {
                self.ip += 1;
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_x) => {
                        if input.len() as i64 - 1 >= 0 {
                            while input.chars().nth(input.len() - 1).unwrap() == '\n' ||
                                input.chars().nth(input.len() - 1).unwrap() == '\r' {
                                    input.pop();
                                    if input.len() as i64 - 1 < 0 {
                                        break;
                                    }
                                }
                        }
                    },
                    Err(_error) => input = "".to_string(),
                }
                if self.code[self.ip] == INT {
                    let result: i64;
                    match input.parse::<i64>() {
                        Ok(x) => result = x,
                        Err(_error) => result = 0,
                    }
                    self.stack.push(result);
                } else if self.code[self.ip] == FLOAT {
                    let result: f64;
                    match input.parse::<f64>() {
                        Ok(x) => result = x,
                        Err(_error) => result = 0.0,
                    }
                    self.stack.push(i64::from_be_bytes(result.to_be_bytes()));
                } else if self.code[self.ip] == STRING {
                    self.string_constants.push(input.clone());
                    self.stack.push(self.string_constants.len() as i64 - 1);
                }
                self.sp += 1;
                self.ip += 1;
            },
            STRING_TO_INT => {
                self.ip += 1;
                let mem_location = self.stack[self.sp - 1] as usize;
                self.stack.pop();

                match self.string_constants[mem_location].parse::<i64>() {
                    Ok(x) => self.stack.push(x),
                    Err(_x) => self.stack.push(0),
                }
            },
            STRING_TO_FLOAT => {
                self.ip += 1;
                let mem_location = self.stack[self.sp - 1] as usize;
                self.stack.pop();

                match self.string_constants[mem_location].parse::<f64>() {
                    Ok(x) => self.stack.push(i64::from_be_bytes(x.to_be_bytes())),
                    Err(_x) => self.stack.push(0),
                }
            },
            INT_TO_FLOAT => {
                self.ip += 1;
                let integer = self.stack[self.sp - 1];
                self.stack.pop();
                let float = integer as f64;
                self.stack.push(i64::from_be_bytes(float.to_be_bytes()));
            },
            INT_TO_STRING => {
                self.ip += 1;
                let integer = self.stack[self.sp - 1];
                self.stack.pop();
                let string = integer.to_string();
                self.string_constants.push(string);
                self.stack.push(self.string_constants.len() as i64 - 1);
            },
            FLOAT_TO_INT => {
                self.ip += 1;
                let float = f64::from_be_bytes(self.stack[self.sp - 1].to_be_bytes());
                self.stack.pop();
                let integer = float as i64;
                self.stack.push(integer);
            },
            FLOAT_TO_STRING => {
                self.ip += 1;
                let float = f64::from_be_bytes(self.stack[self.sp - 1].to_be_bytes());
                self.stack.pop();
                let string = float.to_string();
                self.string_constants.push(string);
                self.stack.push(self.string_constants.len() as i64 - 1);
            },
            GET_STRING_INDEX => {
                self.ip += 1;
                let string_mem_location = self.stack[self.sp - 2] as usize;
                let index = self.stack[self.sp - 1] as usize;
                self.stack.pop();
                self.stack.pop();
                let mut new_string = String::new();
                match self.string_constants[string_mem_location].chars().nth(index) {
                    Some(x) => new_string.push(x),
                    None => {},
                }
                self.string_constants.push(new_string);
                self.stack.push(self.string_constants.len() as i64 - 1);
                self.sp -= 1;
            },
            SET_STRING_INDEX => {
                self.ip += 1;
                let string_mem_location = self.stack[self.sp - 3] as usize;
                let index = self.stack[self.sp - 2] as usize;
                let char_mem_location = self.stack[self.sp - 1] as usize;
                self.stack.pop();
                self.stack.pop();
                self.stack.pop();
                match self.string_constants[char_mem_location].chars().nth(0) {
                    Some(x) => self.string_constants[string_mem_location].insert(index, x),
                    None => {},
                }
                self.sp -= 3;
            },
            GET_COPY_STRING => {
                self.ip += 1;
                let string_mem_location = self.stack[self.sp - 1] as usize;
                self.stack.pop();
                let new_string = self.string_constants[string_mem_location].clone();
                self.string_constants.push(new_string);
                self.stack.push(self.string_constants.len() as i64 - 1);
            },
            _ => {
                panic!("Standard library function does not exist.");
            },
        }
    }
}