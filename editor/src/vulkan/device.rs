use std::collections::HashSet;

use anyhow::{Ok, Result};
use vulkanalia::{
  vk::{self, DeviceV1_0, HasBuilder},
  Device, Entry, Instance,
};

use super::{
  physical_device::DEVICE_EXTENSIONS, validation_vk::VALIDATION_ENABLED, VulkanAppData, PORTABILITY_MACOS_VERSION,
};
use crate::vulkan::{queue_family::QueueFamilyIndices, validation_vk::VALIDATION_LAYER};

pub unsafe fn create_logical(entry: &Entry, instance: &Instance, data: &mut VulkanAppData) -> Result<Device> {
  let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

  let mut unique_indices = HashSet::new();
  unique_indices.insert(indices.graphics);
  unique_indices.insert(indices.present);

  let queue_priorities = &[1.0];
  let queue_infos = unique_indices
    .iter()
    .map(|i| {
      vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(*i)
        .queue_priorities(queue_priorities)
    })
    .collect::<Vec<_>>();

  let layers = if VALIDATION_ENABLED {
    vec![VALIDATION_LAYER.as_ptr()]
  } else {
    vec![]
  };

  let mut extensions = DEVICE_EXTENSIONS.iter().map(|n| n.as_ptr()).collect::<Vec<_>>();

  if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
    extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
  }

  // Features
  let features = vk::PhysicalDeviceFeatures::builder();

  // Create
  let info = vk::DeviceCreateInfo::builder()
    .queue_create_infos(&queue_infos)
    .enabled_layer_names(&layers)
    .enabled_extension_names(&extensions)
    .enabled_features(&features);

  let device = instance.create_device(data.physical_device, &info, None)?;

  // Queues
  data.graphics_queue = device.get_device_queue(indices.graphics, 0);
  data.present_queue = device.get_device_queue(indices.present, 0);

  Ok(device)
}
