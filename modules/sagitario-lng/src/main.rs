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
  chunk.write_chunk(OpCode::RETURN);
  chunk.write_chunk(OpCode::FALSE);
  disassemble_chunk(&chunk, &"test chunk");
  chunk.free_chunk();
}
