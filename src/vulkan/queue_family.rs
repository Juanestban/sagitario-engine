use anyhow::{anyhow, Ok, Result};
use vulkanalia::{
  vk::{self, InstanceV1_0},
  Instance,
};

use super::{utils::sagitario_error::SuitabilityError, VulkanAppData};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct QueueFamilyIndices {
  pub graphics: u32,
}

impl QueueFamilyIndices {
  pub unsafe fn get(instance: &Instance, _data: &VulkanAppData, physical_device: vk::PhysicalDevice) -> Result<Self> {
    let properties = instance.get_physical_device_queue_family_properties(physical_device);

    let graphics = properties
      .iter()
      .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
      .map(|i| i as u32);

    if let Some(graphics) = graphics {
      Ok(Self { graphics })
    } else {
      Err(anyhow!(SuitabilityError("Missing required queue families.")))
    }
  }
}
