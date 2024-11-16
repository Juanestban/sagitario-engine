use std::collections::HashSet;

use anyhow::{anyhow, Ok, Result};
use log::{info, warn};
use vulkanalia::{
  vk::{self, InstanceV1_0},
  Instance,
};

use super::queue_family::QueueFamilyIndices;
use super::utils::sagitario_error::SuitabilityError;
use super::{spawnchair::SwapchainSupport, VulkanAppData};

pub const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];

pub unsafe fn pick_physical_device(instance: &Instance, data: &mut VulkanAppData) -> Result<()> {
  for physical_device in instance.enumerate_physical_devices()? {
    let properties = instance.get_physical_device_properties(physical_device);

    if let Err(error) = check_physical_device(instance, data, physical_device) {
      warn!("Skipping physical device (`{}`): {}", properties.device_name, error);
    } else {
      info!("Selected physical device (`{}`).", properties.device_name);
      data.physical_device = physical_device;

      return Ok(());
    }
  }

  Err(anyhow!("Failed to find suitable physical device."))
}

pub unsafe fn check_physical_device(
  instance: &Instance,
  data: &VulkanAppData,
  physical_device: vk::PhysicalDevice,
) -> Result<()> {
  QueueFamilyIndices::get(instance, data, physical_device)?;
  check_physical_device_extensions(instance, physical_device)?;

  let support = SwapchainSupport::get(instance, data, physical_device)?;

  if support.formats.is_empty() || support.present_modes.is_empty() {
    return Err(anyhow!(SuitabilityError("Insufficient swapchair support.")));
  }

  Ok(())
}

unsafe fn check_physical_device_extensions(instance: &Instance, physical_device: vk::PhysicalDevice) -> Result<()> {
  let extensions = instance
    .enumerate_device_extension_properties(physical_device, None)?
    .iter()
    .map(|e| e.extension_name)
    .collect::<HashSet<_>>();

  if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
    Ok(())
  } else {
    Err(anyhow!(SuitabilityError("Missing required device extensions.")))
  }
}
