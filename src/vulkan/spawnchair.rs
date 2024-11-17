use anyhow::{Ok, Result};
use vulkanalia::{
  vk::{self, HasBuilder, KhrSurfaceExtension, KhrSwapchainExtension},
  Device, Instance,
};
use vulkanalia_sys::Handle;
use winit::window::Window;

use super::{queue_family::QueueFamilyIndices, VulkanAppData};

#[derive(Clone, Debug)]
pub struct SwapchainSupport {
  pub capabilities: vk::SurfaceCapabilitiesKHR,
  pub formats: Vec<vk::SurfaceFormatKHR>,
  pub present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapchainSupport {
  pub unsafe fn get(instance: &Instance, data: &VulkanAppData, physical_device: vk::PhysicalDevice) -> Result<Self> {
    Ok(Self {
      capabilities: instance.get_physical_device_surface_capabilities_khr(physical_device, data.surface)?,
      formats: instance.get_physical_device_surface_formats_khr(physical_device, data.surface)?,
      present_modes: instance.get_physical_device_surface_present_modes_khr(physical_device, data.surface)?,
    })
  }
}

pub unsafe fn create_swapchain(
  window: &Window,
  instance: &Instance,
  device: &Device,
  data: &mut VulkanAppData,
) -> Result<()> {
  let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;
  let support = SwapchainSupport::get(instance, data, data.physical_device)?;

  let surface_format = get_swapchain_surface_format(&support.formats);
  let present_mode = get_swapchain_present_mode(&support.present_modes);
  let extent = get_swapchain_extent(window, support.capabilities);

  let mut image_count = support.capabilities.min_image_count + 1;

  if support.capabilities.max_image_count != 0 && image_count > support.capabilities.max_image_count {
    image_count = support.capabilities.max_image_count;
  }

  let mut queue_family_indices = vec![];
  let image_sharing_mode = if indices.graphics != indices.present {
    queue_family_indices.push(indices.graphics);
    queue_family_indices.push(indices.present);

    vk::SharingMode::CONCURRENT
  } else {
    vk::SharingMode::EXCLUSIVE
  };

  let info = vk::SwapchainCreateInfoKHR::builder()
    .surface(data.surface)
    .min_image_count(image_count)
    .image_format(surface_format.format)
    .image_color_space(surface_format.color_space)
    .image_extent(extent)
    .image_array_layers(1)
    .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
    .image_sharing_mode(image_sharing_mode)
    .queue_family_indices(&queue_family_indices)
    .pre_transform(support.capabilities.current_transform)
    .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
    .present_mode(present_mode)
    .clipped(true)
    .old_swapchain(vk::SwapchainKHR::null());

  data.swapchain_format = surface_format.format;
  data.swapchain_extent = extent;
  data.swapchain = device.create_swapchain_khr(&info, None)?;
  data.swapchain_images = device.get_swapchain_images_khr(data.swapchain)?;

  Ok(())
}

pub fn get_swapchain_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
  formats
    .iter()
    .cloned()
    .find(|f| f.format == vk::Format::B8G8R8A8_SRGB && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR)
    .unwrap_or_else(|| formats[0])
}

pub fn get_swapchain_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
  present_modes
    .iter()
    .cloned()
    .find(|m| *m == vk::PresentModeKHR::MAILBOX)
    .unwrap_or(vk::PresentModeKHR::FIFO)
}

pub fn get_swapchain_extent(window: &Window, capabilities: vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
  if capabilities.current_extent.width != u32::MAX {
    capabilities.current_extent
  } else {
    vk::Extent2D::builder()
      .width(
        window
          .inner_size()
          .width
          .clamp(capabilities.min_image_extent.width, capabilities.max_image_extent.width),
      )
      .height(window.inner_size().height.clamp(
        capabilities.min_image_extent.height,
        capabilities.max_image_extent.height,
      ))
      .build()
  }
}
