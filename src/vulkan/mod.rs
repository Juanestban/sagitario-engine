use anyhow::{anyhow, Ok, Result};
use log::info;
use spawnchain::{create_swapchain, create_swapchain_image_views};
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::KhrSwapchainExtension;
use vulkanalia::vk::{ExtDebugUtilsExtension, KhrSurfaceExtension};
use vulkanalia::window as vk_window;
use vulkanalia::{
  vk::{self, HasBuilder},
  Device, Entry, Instance,
};
use winit::window::Window;

// check vulkan version
use vulkanalia::Version;

// vk-sagitario
pub mod device;
pub mod physical_device;
pub mod pipe;
pub mod queue_family;
pub mod spawnchain;
pub mod utils;
pub mod validation_vk;

use device::create_logical as create_logical_device;
use physical_device::pick_physical_device;
use pipe::{create_pipeline, render_pass::create_render_pass};
use validation_vk::{debug_callback, validations_layers, VALIDATION_ENABLED};

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

#[allow(dead_code)]
pub struct VulkanApp {
  entry: Entry,
  instance: Instance,
  data: VulkanAppData,
  device: Device,
}

pub struct VulkanAppData {
  messenger: vk::DebugUtilsMessengerEXT,
  surface: vk::SurfaceKHR,
  physical_device: vk::PhysicalDevice,
  graphics_queue: vk::Queue,
  present_queue: vk::Queue,
  swapchain_format: vk::Format,
  swapchain_extent: vk::Extent2D,
  swapchain: vk::SwapchainKHR,
  swapchain_images: Vec<vk::Image>,
  swapchain_images_views: Vec<vk::ImageView>,
  render_pass: vk::RenderPass,
  pipeline_layout: vk::PipelineLayout,
  pipeline: vk::Pipeline,
}

impl Default for VulkanAppData {
  fn default() -> Self {
    Self {
      messenger: vk::DebugUtilsMessengerEXT::default(),
      physical_device: vk::PhysicalDevice::default(),
      graphics_queue: vk::Queue::default(),
      surface: vk::SurfaceKHR::default(),
      present_queue: vk::Queue::default(),
      swapchain_format: vk::Format::default(),
      swapchain_extent: vk::Extent2D::default(),
      swapchain: vk::SwapchainKHR::default(),
      swapchain_images: Vec::default(),
      swapchain_images_views: Vec::default(),
      pipeline_layout: vk::PipelineLayout::default(),
      render_pass: vk::RenderPass::default(),
      pipeline: vk::Pipeline::default(),
    }
  }
}

impl VulkanApp {
  pub unsafe fn create(window: &Window) -> Result<Self> {
    info!("[+] VulkanApp::create -> starting");

    let loader = LibloadingLoader::new(LIBRARY)?;
    let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
    let mut data = VulkanAppData::default();
    let instance = create_vk_instance(window, &entry, &mut data)?;
    data.surface = vk_window::create_surface(&instance, &window, &window)?;

    pick_physical_device(&instance, &mut data)?;

    let device = create_logical_device(&entry, &instance, &mut data)?;

    create_swapchain(window, &instance, &device, &mut data)?;
    create_swapchain_image_views(&device, &mut data)?;
    create_render_pass(&instance, &device, &mut data)?;
    create_pipeline(&device, &mut data)?;

    Ok(Self {
      entry,
      instance,
      data,
      device,
    })
  }

  pub unsafe fn render(&mut self, _window: &Window) -> Result<()> {
    Ok(())
  }

  pub unsafe fn destroy(&mut self) {
    self.device.destroy_pipeline(self.data.pipeline, None);
    self.device.destroy_pipeline_layout(self.data.pipeline_layout, None);
    self.device.destroy_render_pass(self.data.render_pass, None);
    self
      .data
      .swapchain_images_views
      .iter()
      .for_each(|v| self.device.destroy_image_view(*v, None));

    self.device.destroy_swapchain_khr(self.data.swapchain, None);
    self.device.destroy_device(None);
    self.instance.destroy_surface_khr(self.data.surface, None);

    if VALIDATION_ENABLED {
      self
        .instance
        .destroy_debug_utils_messenger_ext(self.data.messenger, None);
    }

    self.instance.destroy_instance(None);
  }
}

unsafe fn create_vk_instance(window: &Window, entry: &Entry, data: &mut VulkanAppData) -> Result<Instance> {
  info!("[+] creating_vk_instance");

  let application_info = vk::ApplicationInfo::builder()
    .application_name(b"Sagitario Engine\0")
    .application_version(vk::make_version(1, 0, 0))
    .engine_name(b"SagitarioEngine\0")
    .engine_version(vk::make_version(1, 0, 0))
    .api_version(vk::make_version(1, 0, 0));

  let layers = validations_layers(entry)?;
  let mut extensions = vk_window::get_required_instance_extensions(window)
    .iter()
    .map(|e| e.as_ptr())
    .collect::<Vec<_>>();

  // Only for macOS
  // Required by Vulkan SDK on macOS since 1.3.216.
  let flags = if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
    info!("[INFO]: Enabling extensions for macOS");
    extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
    extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
    vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
  } else {
    vk::InstanceCreateFlags::empty()
  };

  if VALIDATION_ENABLED {
    extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
  }

  let mut info = vk::InstanceCreateInfo::builder()
    .application_info(&application_info)
    .enabled_layer_names(&layers)
    .enabled_extension_names(&extensions)
    .flags(flags);

  let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
    .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
    .message_type(
      vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
    )
    .user_callback(Some(debug_callback));

  if VALIDATION_ENABLED {
    info = info.push_next(&mut debug_info);
  }

  let instance = entry.create_instance(&info, None)?;

  if VALIDATION_ENABLED {
    data.messenger = instance.create_debug_utils_messenger_ext(&debug_info, None)?;
  }

  Ok(instance)
}
