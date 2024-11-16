use anyhow::{anyhow, Ok, Result};
use log::{info, warn};
use vulkanalia::{
  vk::{self, InstanceV1_0},
  Instance,
};

// use super::utils::sagitario_error::SuitabilityError;
use super::queue_family::QueueFamilyIndices;
use super::VulkanAppData;

pub unsafe fn pick(instance: &Instance, data: &mut VulkanAppData) -> Result<()> {
  for physical_device in instance.enumerate_physical_devices()? {
    let properties = instance.get_physical_device_properties(physical_device);

    if let Err(error) = check(instance, data, physical_device) {
      warn!("Skipping physical device (`{}`): {}", properties.device_name, error);
    } else {
      info!("Selected physical device (`{}`).", properties.device_name);
      data.physical_device = physical_device;

      return Ok(());
    }
  }

  Err(anyhow!("Failed to find suitable physical device."))
}

pub unsafe fn check(instance: &Instance, data: &VulkanAppData, physical_device: vk::PhysicalDevice) -> Result<()> {
  QueueFamilyIndices::get(instance, data, physical_device)?;
  Ok(())
}
