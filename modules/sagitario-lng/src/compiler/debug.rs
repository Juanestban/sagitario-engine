use super::csg_chunk::{Chunk, OpCode};
use super::value::print_value;
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

  if offset > 0 && unsafe { *chunk.lines.add(offset) == *chunk.lines.add(offset - 1) } {
    info(vec!["   | "]);
  } else {
    info(vec![format!("{:>4}", unsafe { *chunk.lines.add(offset) })]);
  }

  let instruction = unsafe { *chunk.code.add(offset) };

  match instruction {
    x if x == OpCode::CONSTANT as u8 => constant_instruction("CONSTANT", chunk, offset),
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

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
  let constant = unsafe { *chunk.code.add(offset + 1) };
  let constant_string = constant.to_string();

  info(vec![format!("{:<16} {:>4} '", name, constant_string.as_str())]);

  let value = unsafe { *chunk.constants.values.add(constant as usize) };

  print_value(value);

  offset + 2
}
