use std::fmt::Display;
use std::process;

use super::stack::AlaybeyStack;

#[derive(Debug)]
pub enum VMError<'a> {
    DivZero,
    Halt,
    Undefined,
    Unknown(&'a Type),
    StackErr(String),
}

impl<'a> Display for VMError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VMError::DivZero => write!(f, "math error! data cannot divide zero.."),
            VMError::Halt => write!(f, "incomplete code execution, memory boundry reached without reading HALT instruction.."),
            VMError::Undefined => write!(f, "[exec]: Undefined instruction"),
            VMError::Unknown(typ)=> write!(f,"unknown instruction type error! instruction code:{:?}",typ),
            VMError::StackErr(msg) => write!(f, "{}",msg),            
        }
    }
}

pub type VMResult<'a, T> = Result<T, VMError<'a>>;

#[derive(Debug)]
pub enum Type {
    DataPositive,
    DataNegative,
    Nstruction,
    VariableDef,
    VariableUsage,
    Unknown,
}
#[derive(Debug)]
enum OpCode {
    ADD,
    SUB,
    MUL,
    DiV,
    MOD,
    HALT,
    SemiColon,
    OpenBracket,
    CloseBracket,
    DefineVariable,
    Unknown,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct AlaybeyVM {
    program_counter: usize,
    instruction_type: Type,
    program_data: i32,
    is_runnig: bool,
    vm_stack: AlaybeyStack,
    program_memory: Vec<i32>,
    has_variable_def: bool,
    variable_def_name: i32,
    output: Vec<String>,
}

impl AlaybeyVM {
    pub fn new(stack_size: usize) -> Self {
        AlaybeyVM {
            program_counter: 0,
            instruction_type: Type::Unknown,
            program_data: 0,
            is_runnig: false,
            vm_stack: AlaybeyStack::new(stack_size),
            program_memory: Vec::new(),
            output: Vec::new(),
            has_variable_def: false,
            variable_def_name: 0,
        }
    }

    fn is_runnig(&self) -> bool {
        self.is_runnig
    }

    fn set_runnig(&mut self, state: bool) {
        self.is_runnig = state
    }
    fn get_program_data(&self) -> i32 {
        self.program_data
    }
    fn get_instruction_type(&self) -> &Type {
        &self.instruction_type
    }

    pub fn load_program(&mut self, instructions: &[i32]) {
        self.program_memory.push(0xDEFC0DE); // init code, so program counter start with 1
        for instruction in instructions {
            self.program_memory.push(*instruction);
        }
    }

    pub fn run(&mut self) -> VMResult<()> {
        println!("Memory content : {:?}", self.program_memory);
        self.set_runnig(true);
        while self.is_runnig() {
            if let Err(e) = self.fetch() {
                eprintln!("Fetch Error : {}", e);
                process::exit(0)
            }
            if let Err(e) = self.decode() {
                eprintln!("Decode Error : {}", e);
                process::exit(0)
            }
            if let Err(e) = self.exec() {
                eprintln!("Exec Error : {}", e);
                process::exit(0)
            }
        }
        for out in &self.output {
            println!("{}", out);
        }
        Ok(())
    }

    fn fetch(&mut self) -> VMResult<()> {
        if self.program_counter < self.program_memory.len() {
            self.program_counter += 1
        } else {
            return Err(VMError::Halt);
        }
        Ok(())
    }

    fn decode_data(instruction: u32) -> i32 {
        (instruction & 0xffffffff) as i32
    }
    fn decode_instruction(instruction: i32) -> i32 {
        instruction & 0xfffffff
    }

    fn decode_instruction_type(instruction: i32) -> OpCode {
        let decode = AlaybeyVM::decode_instruction(instruction);
        match decode {
            0 => OpCode::HALT,
            1 => OpCode::ADD,
            2 => OpCode::SUB,
            3 => OpCode::MUL,
            4 => OpCode::DiV,
            5 => OpCode::MOD,
            6 => OpCode::SemiColon,
            7 => OpCode::OpenBracket,
            8 => OpCode::CloseBracket,
            9 => OpCode::DefineVariable,
            _ => OpCode::Unknown,
        }
    }

    fn decode_type(instruction: u32) -> Type {
        //instruction:
        // 0xE0000000 : 11100000000000000000000000000000
        // 0x40000007 : 1000000000000000000000000000111
        // &          : 1000000000000000000000000000000
        // >> 30      : 1>>000000000000000000000000000000
        // >> 29      : 10>>00000000000000000000000000000
        // result     : 10 => 0*2^0 + 1*2^1 = 2
        //variable def:
        //0xE0000000  : 11100000000000000000000000000000
        //0xA0000001  : 10100000000000000000000000000001 (def variable)
        // &          : 10100000000000000000000000000000
        // >> 30      : 10>>100000000000000000000000000000
        // >> 29      : 101>>00000000000000000000000000000
        // result     : 101 => 1*2^0 + 0*2^1 + 1*2^2 = 5
        //variable usage:
        //0xE0000000  : 11100000000000000000000000000000
        //0x60000001  : 01100000000000000000000000000001 (def variable)
        // &          : 01100000000000000000000000000000
        // >> 30      : 01>>100000000000000000000000000000
        // >> 29      : 011>>00000000000000000000000000000
        // result     : 011 => 1*2^0 + 1*2^1 + 0*2^2 =3
        let decode_type = ((instruction & 0xE0000000) >> 29) as u8;
        match decode_type {
            0 => Type::DataPositive,
            2 => Type::Nstruction,
            3 => Type::VariableUsage,
            5 => Type::VariableDef,
            7 => Type::DataNegative,
            _ => Type::Unknown,
        }
    }

    fn decode(&mut self) -> VMResult<()> {
        let word = self.program_memory[self.program_counter];
        self.instruction_type = AlaybeyVM::decode_type(word as u32);
        match self.instruction_type {
            Type::DataPositive => self.program_data = AlaybeyVM::decode_instruction(word),
            Type::VariableDef => {
                self.program_data = AlaybeyVM::decode_instruction(word);
                self.variable_def_name = self.program_data;
            }
            Type::VariableUsage => {
                self.program_data = AlaybeyVM::decode_instruction(word);
            }
            Type::DataNegative | Type::Nstruction => {
                self.program_data = AlaybeyVM::decode_data(word as u32)
            }
            Type::Unknown => return Err(VMError::Unknown(&self.instruction_type)),
        }
        Ok(())
    }

    fn exec(&mut self) -> VMResult<()> {
        Ok(match self.get_instruction_type() {
            Type::Nstruction => self.stack_opearation()?,
            Type::DataPositive | Type::DataNegative => {
                if let Err(e) = self
                    .vm_stack
                    .push_data((None, (Some(self.program_data), None)))
                {
                    return Err(VMError::StackErr(e.to_string()));
                }
                println!(
                    "type :  DATA (type:{:?} = data:{})",
                    self.get_instruction_type(),
                    self.get_program_data()
                );
            }
            Type::VariableUsage => {
                if let Err(e) = self
                    .vm_stack
                    .push_data((None, (None, Some(self.program_data as u8))))
                {
                    return Err(VMError::StackErr(e.to_string()));
                }
                println!(
                    "type :  POiNTER (type:{:?} = pointer address:{})",
                    self.get_instruction_type(),
                    self.get_program_data()
                );
            }
            Type::VariableDef => return Ok(()),
            Type::Unknown => return Err(VMError::Unknown(&self.get_instruction_type())), //panic!("unknown instruction type error! instruction code : {:?}",self.get_instruction_type()),
        })
    }
    fn stack_opearation(&mut self) -> VMResult<()> {
        let op_code = AlaybeyVM::decode_instruction_type(self.get_program_data());
        println!("type : NSTRUCTON, operation:{:?}", op_code);
        match op_code {
            OpCode::HALT => {
                println!("[HALT]: Stopping VM");
                self.set_runnig(false);
            }
            OpCode::ADD => {
                self.operate(OpCode::ADD)?;
            }
            OpCode::SUB => {
                self.operate(OpCode::SUB)?;
            }

            OpCode::MUL => {
                self.operate(OpCode::MUL)?;
            }

            OpCode::DiV => {
                self.operate(OpCode::DiV)?;
            }

            OpCode::MOD => {
                self.operate(OpCode::MOD)?;
            }

            OpCode::SemiColon => {
                if self.has_variable_def {
                    let mut get_result = self.vm_stack.pop_data_().unwrap();
                    get_result.0 = Some(self.variable_def_name as u8);
                    if let Err(e) = self.vm_stack.push_data(get_result) {
                        return Err(VMError::StackErr(e.to_string()));
                    }
                    self.has_variable_def = false;
                }
                self.output.push(format!(
                    "Line RESULT: {}",
                    self.vm_stack.peek_data_().unwrap().1 .0.unwrap()
                ));
                println!("[Semicolon]");
            }
            OpCode::OpenBracket => {
                println!("[Open Bracket]")
            }
            OpCode::CloseBracket => {
                println!("[Close Bracket]")
            }
            OpCode::DefineVariable => {
                self.has_variable_def = true;
                println!("[Define Variable]");
            }
            OpCode::Unknown => {
                return Err(VMError::Undefined);
            }
        };
        Ok(())
    }

    fn operate(&mut self, opcode: OpCode) -> VMResult<()> {
        let (top_1, top_2);
        let (_, (vm_data, pointer_addr)) = match self.vm_stack.pop_data_() {
            Ok(data) => data,
            Err(e) => return Err(VMError::StackErr(e.to_string())),
        };

        if let Some(ptr) = pointer_addr {
            top_1 = match self.vm_stack.get_data_from_pointer_(ptr) {
                Ok(data) => data,
                Err(e) => return Err(VMError::StackErr(e.to_string())),
            }
        } else if let Some(top1) = vm_data {
            top_1 = top1;
        } else {
            return Err(VMError::StackErr("unable to get data..".to_string()));
        }
        let (_, (vm_data, pointer_addr)) = match self.vm_stack.pop_data_() {
            Ok(data) => data,
            Err(e) => return Err(VMError::StackErr(e.to_string())),
        };
        if let Some(ptr) = pointer_addr {
            top_2 = match self.vm_stack.get_data_from_pointer_(ptr) {
                Ok(data) => data,
                Err(e) => return Err(VMError::StackErr(e.to_string())),
            }
        } else {
            if let Some(top2) = vm_data {
                top_2 = top2
            } else {
                return Err(VMError::StackErr("unable to get data..".to_string()));
            }
        }
        match opcode {
            OpCode::ADD => {
                let push_result = (None, (Some(top_2 + top_1), None));
                if let Err(e) = self.vm_stack.push_data(push_result) {
                    return Err(VMError::StackErr(e.to_string()));
                }
                println!("[ADD]: {} {}", top_2, top_1);
            }
            OpCode::SUB => {
                let push_result = (None, (Some(top_2 - top_1), None));
                if let Err(e) = self.vm_stack.push_data(push_result) {
                    return Err(VMError::StackErr(e.to_string()));
                }
                println!("[SUB]: {} {}", top_2, top_1);
            }
            OpCode::MUL => {
                let push_result = (None, (Some(top_2 * top_1), None));
                if let Err(e) = self.vm_stack.push_data(push_result) {
                    return Err(VMError::StackErr(e.to_string()));
                }
                println!("[MUL]: {} {}", top_2, top_1);
            }
            OpCode::DiV => {
                let push_result = (None, (Some(top_2 / top_1), None));
                if let Err(e) = self.vm_stack.push_data(push_result) {
                    return Err(VMError::StackErr(e.to_string()));
                }
                println!("[DiV]: {} {}", top_2, top_1);
            }
            OpCode::MOD => {
                let push_result = (None, (Some(top_2 % top_1), None));
                if let Err(e) = self.vm_stack.push_data(push_result) {
                    return Err(VMError::StackErr(e.to_string()));
                }
                println!("[MOD]: {} {}", top_2, top_1);
            }
            _ => {}
        };
        Ok(())
    }
}
