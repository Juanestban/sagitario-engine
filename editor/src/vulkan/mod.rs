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
pub mod commands;
pub mod device;
pub mod framebuffers;
pub mod physical_device;
pub mod pipe;
pub mod queue_family;
pub mod semaphore;
pub mod spawnchain;
pub mod utils;
pub mod validation_vk;

use commands::{create_command_buffers, create_command_pool};
use device::create_logical as create_logical_device;
use framebuffers::create_framebuffers;
use physical_device::pick_physical_device;
use pipe::{create_pipeline, render_pass::create_render_pass};
use semaphore::create_sync_objects;
use validation_vk::{debug_callback, validations_layers, VALIDATION_ENABLED};

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);
const MAX_FRAMES_IN_FLIGHT: usize = 2;

#[allow(dead_code)]
pub struct VulkanApp {
  entry: Entry,
  instance: Instance,
  data: VulkanAppData,
  device: Device,
  frame: usize,
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
  framebuffers: Vec<vk::Framebuffer>,
  command_pool: vk::CommandPool,
  command_buffers: Vec<vk::CommandBuffer>,
  image_available_semaphore: Vec<vk::Semaphore>,
  render_finished_semaphore: Vec<vk::Semaphore>,
  in_flight_fences: Vec<vk::Fence>,
  images_in_flight: Vec<vk::Fence>,
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
      command_pool: vk::CommandPool::default(),
      framebuffers: Vec::default(),
      command_buffers: Vec::default(),
      image_available_semaphore: Vec::default(),
      render_finished_semaphore: Vec::default(),
      in_flight_fences: Vec::default(),
      images_in_flight: Vec::default(),
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
    create_framebuffers(&device, &mut data)?;
    create_command_pool(&instance, &device, &mut data)?;
    create_command_buffers(&device, &mut data)?;
    create_sync_objects(&device, &mut data)?;

    Ok(Self {
      entry,
      instance,
      data,
      device,
      frame: 0,
    })
  }

  pub unsafe fn render(&mut self, _window: &Window) -> Result<()> {
    let in_flight_fence = self.data.in_flight_fences[self.frame];

    self.device.wait_for_fences(&[in_flight_fence], true, u64::MAX)?;

    let image_index = self
      .device
      .acquire_next_image_khr(
        self.data.swapchain,
        u64::MAX,
        self.data.image_available_semaphore[self.frame],
        vk::Fence::null(),
      )?
      .0 as usize;

    let image_in_flight = self.data.images_in_flight[image_index];

    if !image_in_flight.is_null() {
      self.device.wait_for_fences(&[image_in_flight], true, u64::MAX)?;
    }

    self.data.images_in_flight[image_index] = in_flight_fence;

    let wait_semaphores = &[self.data.image_available_semaphore[self.frame]];
    let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
    let command_buffers = &[self.data.command_buffers[image_index]];
    let signal_semaphores = &[self.data.render_finished_semaphore[self.frame]];
    let submit_info = vk::SubmitInfo::builder()
      .wait_semaphores(wait_semaphores)
      .wait_dst_stage_mask(wait_stages)
      .command_buffers(command_buffers)
      .signal_semaphores(signal_semaphores);

    self.device.reset_fences(&[in_flight_fence])?;

    self
      .device
      .queue_submit(self.data.graphics_queue, &[submit_info], in_flight_fence)?;

    let swapchains = &[self.data.swapchain];
    let image_indices = &[image_index as u32];
    let present_info = vk::PresentInfoKHR::builder()
      .wait_semaphores(signal_semaphores)
      .swapchains(swapchains)
      .image_indices(image_indices);

    self.device.queue_present_khr(self.data.present_queue, &present_info)?;

    self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;

    Ok(())
  }

  pub unsafe fn destroy(&mut self) {
    self.device.device_wait_idle().unwrap();

    self
      .data
      .in_flight_fences
      .iter()
      .for_each(|f| self.device.destroy_fence(*f, None));
    self
      .data
      .render_finished_semaphore
      .iter()
      .for_each(|s| self.device.destroy_semaphore(*s, None));
    self
      .data
      .image_available_semaphore
      .iter()
      .for_each(|s| self.device.destroy_semaphore(*s, None));

    self.device.destroy_command_pool(self.data.command_pool, None);
    self
      .data
      .framebuffers
      .iter()
      .for_each(|f| self.device.destroy_framebuffer(*f, None));
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
