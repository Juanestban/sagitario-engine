use anyhow::{Ok, Result};
use vulkanalia::{
  vk::{self, DeviceV1_0, HasBuilder},
  Device,
};

use super::VulkanAppData;

pub unsafe fn create_framebuffers(device: &Device, data: &mut VulkanAppData) -> Result<()> {
  data.framebuffers = data
    .swapchain_images_views
    .iter()
    .map(|i| {
      let attachments = &[*i];
      let create_info = vk::FramebufferCreateInfo::builder()
        .render_pass(data.render_pass)
        .attachments(attachments)
        .width(data.swapchain_extent.width)
        .height(data.swapchain_extent.height)
        .layers(1);

      device.create_framebuffer(&create_info, None)
    })
    .collect::<Result<Vec<_>, _>>()?;

  Ok(())
}
