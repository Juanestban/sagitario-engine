use anyhow::{anyhow, Ok, Result};
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::window as vk_window;
use vulkanalia::{
  vk::{self, HasBuilder},
  Entry, Instance,
};
use winit::window::Window;

// check vulkan version
use vulkanalia::Version;

// vk-sagitario
pub mod physical_device;
pub mod queue_family;
pub mod utils;
mod validation_vk;

use physical_device::pick as pick_physical_device;
use validation_vk::{debug_callback, validations_layers, VALIDATION_ENABLED};

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

unsafe fn create_vk_instance(window: &Window, entry: &Entry, data: &mut VulkanAppData) -> Result<Instance> {
  let application_info = vk::ApplicationInfo::builder()
    .application_name(b"Sagitario Engine\0")
    .application_version(vk::make_version(1, 0, 0))
    .engine_name(b"SagitarioEngine\0")
    .engine_version(vk::make_version(1, 0, 0))
    .api_version(vk::make_version(1, 0, 0));

  let mut extensions = vk_window::get_required_instance_extensions(window)
    .iter()
    .map(|e| e.as_ptr())
    .collect::<Vec<_>>();

  if VALIDATION_ENABLED {
    extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
  }

  // Only for macOS
  // Required by Vulkan SDK on macOS since 1.3.216.
  let flags = if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
    print!("[INFO]: Enabling extensions for macOS");
    extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
    extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
    vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
  } else {
    vk::InstanceCreateFlags::empty()
  };

  let layers = validations_layers(entry)?;

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

#[allow(dead_code)]
pub struct VulkanApp {
  entry: Entry,
  instance: Instance,
  data: VulkanAppData,
}

pub struct VulkanAppData {
  messenger: vk::DebugUtilsMessengerEXT,
  physical_device: vk::PhysicalDevice,
}

impl Default for VulkanAppData {
  fn default() -> Self {
    Self {
      messenger: vk::DebugUtilsMessengerEXT::default(),
      physical_device: vk::PhysicalDevice::default(),
    }
  }
}

impl VulkanApp {
  pub unsafe fn create(window: &Window) -> Result<Self> {
    let loader = LibloadingLoader::new(LIBRARY)?;
    let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
    let mut data = VulkanAppData::default();
    let instance = create_vk_instance(window, &entry, &mut data)?;

    pick_physical_device(&instance, &mut data)?;

    Ok(Self { entry, instance, data })
  }

  pub unsafe fn render(&mut self, _window: &Window) -> Result<()> {
    Ok(())
  }

  pub unsafe fn destroy(&mut self) {
    if VALIDATION_ENABLED {
      self
        .instance
        .destroy_debug_utils_messenger_ext(self.data.messenger, None);
    }

    self.instance.destroy_instance(None);
  }
}
