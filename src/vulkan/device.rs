use anyhow::{Ok, Result};
use vulkanalia::{
  vk::{self, DeviceV1_0, HasBuilder},
  Device, Entry, Instance,
};

use super::{validation_vk::VALIDATION_ENABLED, VulkanAppData, PORTABILITY_MACOS_VERSION};
use crate::vulkan::{queue_family::QueueFamilyIndices, validation_vk::VALIDATION_LAYER};

pub unsafe fn create_logical(entry: &Entry, instance: &Instance, data: &mut VulkanAppData) -> Result<Device> {
  let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

  let queue_priority = &[1.0];
  let queue_info = vk::DeviceQueueCreateInfo::builder()
    .queue_family_index(indices.graphics)
    .queue_priorities(queue_priority);

  let layers = if VALIDATION_ENABLED {
    vec![VALIDATION_LAYER.as_ptr()]
  } else {
    vec![]
  };

  // Extensions

  let mut extensions = vec![];

  // Required by Vulkan SDK on macOS since 1.3.216.
  if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
    extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
  }

  // Features

  let features = vk::PhysicalDeviceFeatures::builder();

  // Create

  let queue_infos = &[queue_info];
  let info = vk::DeviceCreateInfo::builder()
    .queue_create_infos(queue_infos)
    .enabled_layer_names(&layers)
    .enabled_extension_names(&extensions)
    .enabled_features(&features);

  let device = instance.create_device(data.physical_device, &info, None)?;

  // Queues

  data.graphics_queue = device.get_device_queue(indices.graphics, 0);

  Ok(device)
}
