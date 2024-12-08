use anyhow::{Ok, Result};
use vulkanalia::{
  vk::{self, DeviceV1_0, HasBuilder},
  Device, Instance,
};

use super::{queue_family::QueueFamilyIndices, VulkanAppData};

pub unsafe fn create_command_pool(instance: &Instance, device: &Device, data: &mut VulkanAppData) -> Result<()> {
  let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

  let info = vk::CommandPoolCreateInfo::builder().queue_family_index(indices.graphics);

  data.command_pool = device.create_command_pool(&info, None)?;

  Ok(())
}

pub unsafe fn create_command_buffers(device: &Device, data: &mut VulkanAppData) -> Result<()> {
  let allocate_info = vk::CommandBufferAllocateInfo::builder()
    .command_pool(data.command_pool)
    .level(vk::CommandBufferLevel::PRIMARY)
    .command_buffer_count(data.framebuffers.len() as u32);

  data.command_buffers = device.allocate_command_buffers(&allocate_info)?;

  for (i, command_buffer) in data.command_buffers.iter().enumerate() {
    let info = vk::CommandBufferBeginInfo::builder();

    device.begin_command_buffer(*command_buffer, &info)?;

    let render_area = vk::Rect2D::builder()
      .offset(vk::Offset2D::default())
      .extent(data.swapchain_extent);

    let color_clean_value = vk::ClearValue {
      color: vk::ClearColorValue {
        float32: [0.0, 0.0, 0.0, 1.0],
      },
    };

    let clear_values = &[color_clean_value];
    let info = vk::RenderPassBeginInfo::builder()
      .render_pass(data.render_pass)
      .framebuffer(data.framebuffers[i])
      .render_area(render_area)
      .clear_values(clear_values);

    device.cmd_begin_render_pass(*command_buffer, &info, vk::SubpassContents::INLINE);
    device.cmd_bind_pipeline(*command_buffer, vk::PipelineBindPoint::GRAPHICS, data.pipeline);
    device.cmd_draw(*command_buffer, 3, 1, 0, 0);
    device.cmd_end_render_pass(*command_buffer);

    device.end_command_buffer(*command_buffer)?;
  }

  Ok(())
}
