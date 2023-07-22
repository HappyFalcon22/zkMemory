#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(special_module_name)]
#![allow(unused_mut)]

use core::fmt::Error;
/// Each cell has some multiplier of bits, like Window(x64) for example
const CELL_SIZE: u64 = 8;

/// We used u64 to store everything regarding for trivial machine
/// Every machine have 64 bits in word size except Wasm v128 and EVM 256 bits
struct UntypedValue {
    bits: u64,
}

/// In WebAssembly there are some certain way to interactive with the memory
/// Following this `enum` it defined three possible actions
#[derive(Clone, Debug)]
enum MemoryAction {
    Init,
    Read,
    Write,
}

/// I/O trace for the memory
struct MemoryTrace {
    time_log: u64,
    action: MemoryAction, /// Instruction
    address: UntypedValue,
    value: UntypedValue,
}

/// Raw memory just use for the storage
/// I thinking about time series data for parallelisation
// #[derive(Debug, Clone)]
struct MemoryRaw<'a> {
    ptr: &'a UntypedValue,
    memory_raw: Vec<u8>,
    memory_trace: Vec<MemoryTrace>,
}

/// zkMemory
struct Memory<'a> {
    time_count: u64,
    raw: MemoryRaw<'a>,
    commitment: Vec<MemoryCommitment>,
}

/// Have no idea about memory commitment but it will be great if we use verkle tree commitment
/// To prove the memory
#[derive(Clone, Debug)]
struct MemoryCommitment {
    bits: u64,
}

/// The interface for zkMemory
trait MemoryInterface<'a> {
    fn new(raw_memory: MemoryRaw<'a>) -> Self;
    fn init(&mut self, address: u64) -> Result<MemoryCommitment, Error>;
    fn read(&mut self, address: u64) -> Result<MemoryCommitment, Error>;
    fn write(&mut self, address: u64, chunk: u64) -> Result<MemoryCommitment, Error>;
    fn extract_memory_trace(&mut self) -> Result<Vec<MemoryTrace>, Error>;
}

// impl <'a>Memory<'a> {

// }

impl <'a>MemoryInterface<'a> for Memory<'a> {
    fn init(&mut self, address: u64) -> Result<MemoryCommitment, Error> {
        Err(Error)
    }

    fn new(raw_memory: MemoryRaw<'a>) -> Memory {
        Memory { 
            time_count: 0u64,
            raw: raw_memory, 
            commitment: Vec::new(), 
        }
    }

    fn read(&mut self, address: u64) -> Result<MemoryCommitment, Error> {
        // Check if the address is within the valid memory range
        let memory_size = self.raw.memory_raw.len() as u64;
        if (address >= memory_size) || (address % CELL_SIZE != 0) {
            return Err(Error); // Return an error if the address is out of range
        }
        // Perform the read operation
        let mut data = self.raw.memory_raw[address as usize] as u64;
        for i in address+1..address+CELL_SIZE {
            data *= 256;
            data += self.raw.memory_raw[i as usize] as u64;
        }
        // Push a MemoryTrace to storage
        let trace = MemoryTrace {
            time_log: self.time_count,
            address: UntypedValue { bits: (address) },
            action: MemoryAction::Read,
            value: UntypedValue { bits: data },
        };

        self.raw.memory_trace.push(trace);

        // Increment the time count
        self.time_count += 1;

        // Return the MemoryCommitment with the read data
        Ok(MemoryCommitment { bits: data as u64 })
    }

    fn write(&mut self, address: u64, chunk: u64) -> Result<MemoryCommitment, Error> {
        // Check if the address is within the valid memory range
        let memory_size = self.raw.memory_raw.len() as u64;
        if address + CELL_SIZE >= memory_size {
            return Err(Error); // Return an error if the address is out of range
        }
        let mut temp = chunk;
        // Perform the write operation by updating the memory
        for i in (address..address+CELL_SIZE).rev() {
            println!("{}", i);
            self.raw.memory_raw[i as usize] = (temp % 0x100u64) as u8;
            temp = temp / 0x100u64;
        }
        // Push a MemoryTrace to storage
        let trace = MemoryTrace {
            time_log: self.time_count,
            address: UntypedValue { bits: (address) },
            action: MemoryAction::Write,
            value: UntypedValue { bits: chunk },
        };

        self.raw.memory_trace.push(trace);

        // Increment the time count
        self.time_count += 1;

        // Return the MemoryCommitment with the updated data
        Ok(MemoryCommitment { bits: chunk })
    }

    fn extract_memory_trace(&mut self) -> Result<Vec<MemoryTrace>, Error> {
        let trace = self.raw.memory_trace;
        Ok(trace)
    }
}


fn main() {
    // Create a raw memory instance, for example:
    let raw_memory = MemoryRaw {
        ptr: &UntypedValue { bits: 0 },
        memory_raw: vec![109, 0, 0, 0, 0, 0, 0, 15, 19, 203, 0, 178, 0, 0, 1, 1], // Initialize memory_raw with some initial data
        memory_trace: Vec::new(),
    };

    let mut memory = Memory::new(raw_memory);
    let mut temp = memory.read(8);
    match temp {
        Ok(v) => println!("{}",format!("{:#0x}", v.bits)),
        Err(_) => println!("Huh ?"),     
    }
    temp = memory.write(0, 0x123456789abcdef0);
    match temp {
        Ok(v) => println!("{:?}", v.bits),
        Err(_) => (),     
    }
    println!("{:?}", memory.raw.memory_raw);
    temp = memory.read(0);
    match temp {
        Ok(v) => println!("{:?}", v.bits),
        Err(_) => println!("Huh ?"),     
    }

    let mm_trace = memory.raw.memory_trace;

    for i in &mm_trace {
        println!("({}, {}, {:?}, {:#0x})", i.time_log, i.address.bits, i.action, i.value.bits);
    };
    
}