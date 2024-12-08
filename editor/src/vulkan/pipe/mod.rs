use anyhow::{Ok, Result};
use vulkanalia::{
  vk::{self, DeviceV1_0, HasBuilder},
  Device,
};
use vulkanalia_sys::Handle;

use super::VulkanAppData;

pub mod render_pass;
pub mod shader;

use shader::create_shader_module;

/**
 * Objetivo: Llegar a cargar los binarios, y compilarlos en tiempo de ejecucion del motor grafico
 * con ello poder ver cambios que produzca el usuario al interactuar con los objetos o entidades
 * en la "surface" de vulkan, como un editor grafico orientado a videojuegos
 * (Unity / Godot / Unreal Engine)
 */

pub unsafe fn create_pipeline(device: &Device, data: &mut VulkanAppData) -> Result<()> {
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

  let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder();

  let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
    .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
    .primitive_restart_enable(false);

  let viewport = vk::Viewport::builder()
    .x(0.0)
    .y(0.0)
    .width(data.swapchain_extent.width as f32)
    .height(data.swapchain_extent.height as f32)
    .min_depth(0.0)
    .max_depth(1.0);

  let scissor = vk::Rect2D::builder()
    .offset(vk::Offset2D { x: 0, y: 0 })
    .extent(data.swapchain_extent);

  let viewports = &[viewport];
  let scissors = &[scissor];
  let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
    .viewports(viewports)
    .scissors(scissors);

  let rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
    .depth_clamp_enable(false)
    .rasterizer_discard_enable(false)
    .polygon_mode(vk::PolygonMode::FILL)
    .line_width(1.0)
    .cull_mode(vk::CullModeFlags::BACK)
    .front_face(vk::FrontFace::CLOCKWISE)
    .depth_bias_enable(false);

  let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
    .sample_shading_enable(false)
    .rasterization_samples(vk::SampleCountFlags::_1);

  let attachment = vk::PipelineColorBlendAttachmentState::builder()
    .color_write_mask(vk::ColorComponentFlags::all())
    .blend_enable(false)
    // .src_color_blend_factor(vk::BlendFactor::ONE)  // Optional
    // .dst_color_blend_factor(vk::BlendFactor::ZERO) // Optional
    // .color_blend_op(vk::BlendOp::ADD)              // Optional
    // .src_alpha_blend_factor(vk::BlendFactor::ONE)  // Optional
    // .dst_alpha_blend_factor(vk::BlendFactor::ZERO) // Optional
    // .alpha_blend_op(vk::BlendOp::ADD);             // Optional
  ;
  let attachments = &[attachment];
  let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
    .logic_op_enable(false)
    .logic_op(vk::LogicOp::COPY)
    .attachments(attachments)
    .blend_constants([0.0, 0.0, 0.0, 0.0]);

  let layout_info = vk::PipelineLayoutCreateInfo::builder();

  data.pipeline_layout = device.create_pipeline_layout(&layout_info, None)?;

  let stages = &[vert_stage, frag_stage];
  let info = vk::GraphicsPipelineCreateInfo::builder()
    .stages(stages)
    .vertex_input_state(&vertex_input_state)
    .input_assembly_state(&input_assembly_state)
    .viewport_state(&viewport_state)
    .rasterization_state(&rasterization_state)
    .multisample_state(&multisample_state)
    .color_blend_state(&color_blend_state)
    .layout(data.pipeline_layout)
    .render_pass(data.render_pass)
    .subpass(0)
    // .base_pipeline_handle(vk::Pipeline::null()) // Optional.
    // .base_pipeline_index(-1)                    // Optional.
    ;

  data.pipeline = device
    .create_graphics_pipelines(vk::PipelineCache::null(), &[info], None)?
    .0[0];

  device.destroy_shader_module(vert_shader_module, None);
  device.destroy_shader_module(frag_shader_module, None);

  Ok(())
}
