use core::panic;
use std::{fmt::Display, usize};

#[derive(Debug)]
pub enum StackError {
    Pointer,
    GetData,
    Stackoverflow(usize, usize),
}

impl Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackError::Pointer => write!(f, "unable to get pointer data.."),
            StackError::GetData => write!(f, "unable to get data.."),
            StackError::Stackoverflow(cap, size) => write!(
                f,
                "Stack overflow error.. capacity {} , size {} ",
                cap, size
            ),
        }
    }
}

pub type StackResult<T> = Result<T, StackError>;

type VariableDef = Option<u8>;
type VMData = Option<i32>;
type VariablePointer = Option<u8>;
type StackData = (VariableDef, (VMData, VariablePointer));
pub struct AlaybeyStack {
    stack_data: Vec<StackData>,
    stack_capacity: usize,
    stack_top: usize,
}

impl AlaybeyStack {
    pub fn new(stack_size: usize) -> Self {
        AlaybeyStack {
            stack_data: Vec::with_capacity(stack_size),
            stack_capacity: stack_size,
            stack_top: 0,
        }
    }
    pub fn push_data(&mut self, stack_data: StackData) -> StackResult<()> {
        if self.stack_top == self.stack_capacity {
            return Err(StackError::Stackoverflow(
                self.stack_capacity,
                self.stack_top,
            ));
        }
        self.stack_data.push(stack_data);
        self.stack_top += 1;
        Ok(())
    }

    pub fn pop_data_(&mut self) -> StackResult<StackData> {
        if self.stack_top == 0 {
            panic!("VM Stack underflow error..")
        }
        self.stack_top -= 1;
        if let Some(data) = self.stack_data.pop() {
            Ok(data)
        } else {
             Err(StackError::GetData)
        }
    }

    pub fn peek_data_(&self) -> StackResult<StackData> {
        if let Some(data) = self.stack_data.last() {
            Ok(*data)
        } else {
             Err(StackError::GetData)
        }
    }
    pub fn get_data_from_pointer_(&self, ptr: u8) -> StackResult<i32> {
        Ok(
            match self
                .stack_data
                .iter()
                .filter(|(pointer, (_, _))| {
                    if let Some(data) = *pointer {
                        data == ptr
                    } else {
                        false
                    }
                })
                .collect::<Vec<_>>()
                .pop()
            {
                Some((_, (vm_data, _))) => {
                    if let Some(return_data) = *vm_data {
                        return_data
                    } else {
                        return Err(StackError::Pointer);
                    }
                }
                None => return Err(StackError::Pointer),
            },
        )
    }
}
