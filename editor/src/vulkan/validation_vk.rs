use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;

use anyhow::{anyhow, Error, Result};
use log::*;
use vulkanalia::{
  vk::{self, EntryV1_0},
  Entry,
};

pub const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
pub const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

pub unsafe fn validations_layers(entry: &Entry) -> Result<Vec<*const i8>, Error> {
  let available_layers = entry
    .enumerate_instance_layer_properties()?
    .iter()
    .map(|l| l.layer_name)
    .collect::<HashSet<_>>();

  if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
    return Err(anyhow!("Validation layer requested but not supported."));
  }

  let layers = if VALIDATION_ENABLED {
    vec![VALIDATION_LAYER.as_ptr()]
  } else {
    Vec::new()
  };

  return Ok(layers);
}

pub extern "system" fn debug_callback(
  severity: vk::DebugUtilsMessageSeverityFlagsEXT,
  type_: vk::DebugUtilsMessageTypeFlagsEXT,
  data: *const vk::DebugUtilsMessengerCallbackDataEXT,
  _: *mut c_void,
) -> vk::Bool32 {
  let data = unsafe { *data };
  let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

  if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
    error!("({:?}) {}", type_, message);
  } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
    warn!("({:?}) {}", type_, message);
  } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
    debug!("({:?}) {}", type_, message);
  } else {
    trace!("({:?}) {}", type_, message);
  }

  vk::FALSE
}
