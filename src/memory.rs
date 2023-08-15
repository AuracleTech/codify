/*
mod memory;
use memory::Memory;

    // let mut mem = Memory::new();
 */

use std::collections::HashMap;

pub struct Value {
    pub data: Box<dyn std::any::Any>,
    pub value_type: Type,
}

pub enum Type {
    Integer,
    String,
}

pub struct Memory {
    pub stack: Vec<Value>,
    pub heap: HashMap<usize, Value>,
}

// IMPLEMENT methods for the Memory struct
impl Memory {
    pub fn new() -> Self {
        Memory {
            stack: Vec::new(),
            heap: HashMap::new(),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn allocate(&mut self, address: usize, value: Value) {
        self.heap.insert(address, value);
    }
}
