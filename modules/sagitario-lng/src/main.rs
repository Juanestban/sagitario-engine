/******************************************************************************************************/
/**                                                                                                  **/
/******************************************************************************************************/
/**                                       Sagitario Language                                         **/
/**                                                                                                  **/
/**                                                                                                  **/
/******************************************************************************************************/
// use std::env;
pub mod compiler;

use compiler::csg_chunk::{Chunk, OpCode};
use compiler::debug::disassemble_chunk;
use sagitario_logger as logger;

fn main() {
  logger::note(vec!["[*] Sagitario version: [0.1.0]"]);

  // let args: Vec<String> = env::args().collect();

  // if args.len() == 1 {
  //   logger::danger(vec!["[-] need use file for test!"]);
  //   return;
  // }

  let mut chunk = Chunk::new();
  let constant = chunk.add_constants(1.2) as u8;

  chunk.write_chunk(OpCode::CONSTANT as u8, 123 as u8);
  chunk.write_chunk(constant, 123 as u8);

  chunk.write_chunk(OpCode::RETURN as u8, 123 as u8);
  disassemble_chunk(&chunk, &"test chunk");
  chunk.free_chunk();
}
