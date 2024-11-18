use anyhow::{Ok, Result};
use vulkanalia::vk::{DeviceV1_0, HasBuilder};
use vulkanalia::{vk, Device};

use vulkanalia::bytecode::Bytecode;

#[allow(unused_variables, dead_code)]
fn build() {
  println!("compiler->build")
}

pub unsafe fn create_shader_module(device: &Device, bytecode: &[u8]) -> Result<vk::ShaderModule> {
  let bytecode = Bytecode::new(bytecode).unwrap();

  let info = vk::ShaderModuleCreateInfo::builder()
    .code_size(bytecode.code_size())
    .code(bytecode.code());

  Ok(device.create_shader_module(&info, None)?)
}
