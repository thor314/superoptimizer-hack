use log::trace;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct CPU<const MAX_MEMORY: usize> {
  state: [u64; MAX_MEMORY],
}

impl<const MAX_MEMORY: usize> CPU<MAX_MEMORY> {
  pub fn new() -> Self { CPU { state: [0; MAX_MEMORY] } }

  pub fn execute(&mut self, program: Program) {
    for ins in program.0 {
      match ins {
        Opcode::LOAD { location } => {
          trace!("LOAD");
          self.load(location);
        },
        Opcode::SWAP { mem1, mem2 } => {
          //
          trace!("SWAP");
          self.swap(mem1, mem2);
        },
        Opcode::XOR { mem1, mem2 } => {
          //
          trace!("XOR");
          self.xor(mem1, mem2);
        },
        Opcode::INC{mem} => {
          //
          trace!("INC");
           self.inc(mem);
        },
      }
    }
  }

  fn load(&mut self, location: u64)  {
    self.state[0] = location;
  }

  fn swap(&mut self, mem1: u64, mem2: u64) {
    self.state.swap(mem1 as usize, mem2 as usize);
  }

  fn xor(&mut self, mem1: u64, mem2: u64){
    self.state[mem1 as usize] ^= self.state[mem2 as usize];
  }

  fn inc(&mut self, mem: u64){
    self.state[mem as usize] += 1;
  }
}

/// A program is just a list of opcodes (bro)
pub struct Program (Vec<Opcode>);

/// 
#[allow(clippy::upper_case_acronyms)]
pub enum Opcode {
  LOAD { location: u64 },
  SWAP { mem1: u64, mem2: u64 },
  XOR { mem1: u64, mem2: u64 },
  INC { mem: u64 },
}
