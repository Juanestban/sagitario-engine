use anyhow::{Ok, Result};
use vulkanalia::{
  vk::{self, DeviceV1_0, HasBuilder},
  Device,
};

use super::VulkanAppData;

pub mod shader;

use shader::create_shader_module;

/**
 * Objetivo: Llegar a cargar los binarios, y compilarlos en tiempo de ejecucion del motor grafico
 * con ello poder ver cambios que produzca el usuario al interactuar con los objetos o entidades
 * en la "surface" de vulkan, como un editor grafico orientado a videojuegos
 * (Unity / Godot / Unreal Engine)
 */

pub unsafe fn create_pipeline(device: &Device, _data: &mut VulkanAppData) -> Result<()> {
  let vert = include_bytes!("./shader/.tmp/triangle/vert.spv");
  let frag = include_bytes!("./shader/.tmp/triangle/frag.spv");

  let vert_shader_module = create_shader_module(device, &vert[..])?;
  let frag_shader_module = create_shader_module(device, &frag[..])?;

  // [!stage]
  let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
    .stage(vk::ShaderStageFlags::VERTEX)
    .module(vert_shader_module)
    .name(b"main\0");

  let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
    .stage(vk::ShaderStageFlags::FRAGMENT)
    .module(frag_shader_module)
    .name(b"main\0");

  device.destroy_shader_module(vert_shader_module, None);
  device.destroy_shader_module(frag_shader_module, None);
  Ok(())
}
