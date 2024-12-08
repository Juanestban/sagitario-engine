use super::csg_chunk::{Chunk, OpCode};
use sagitario_logger::info;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
  info(vec![format!("== {} ==", name)]);
  let mut offset: usize = 0;

  while offset < chunk.count {
    offset = disassemble_instruction(chunk, offset);
  }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
  info(vec![format!("{:04}", offset)]);

  let instruction = unsafe { *chunk.code.add(offset) };

  match instruction {
    x if x == OpCode::RETURN as u8 => simple_instruction("RETURN", offset),
    _ => {
      info(vec![format!("unknown opcode {}", instruction)]);

      offset + 1
    }
  }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
  info(vec![format!("{}", name)]);

  offset + 1
}
