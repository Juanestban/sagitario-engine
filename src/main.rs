use anyhow::{Ok, Result};
use image::GenericImageView;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Icon, Theme, Window, WindowId};

mod vulkan;
use vulkan::VulkanApp;
// use vulkan::create_vk_instance;

fn load_icon() -> Result<Icon, Box<dyn std::error::Error>> {
  let icon_path = include_bytes!("./assets/icon.png");
  let image = image::load_from_memory(icon_path)?;
  let (width, height) = image.dimensions();
  let rgba = image.into_rgba8().into_raw();

  // Crear un ícono para winit
  Icon::from_rgba(rgba, width, height).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[derive(Default)]
struct App {
  window: Option<Window>,
  vk_app: Option<VulkanApp>,
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let icon = load_icon().unwrap();

    let custom_window = Window::default_attributes()
      .with_theme(Some(Theme::Dark))
      .with_title("Sagitario Engine")
      .with_inner_size(LogicalSize::new(800, 600))
      .with_window_icon(Some(icon))
      .with_active(true);

    self.window = Some(event_loop.create_window(custom_window).unwrap());
    self.vk_app = Some(unsafe { VulkanApp::create(self.window.as_ref().unwrap()) }.unwrap());
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        print!("The close button was pressed, stopping");
        event_loop.exit();
        unsafe {
          self.vk_app.as_mut().unwrap().destroy();
        }
      }
      WindowEvent::RedrawRequested => {
        self.window.as_ref().unwrap().request_redraw();

        unsafe { self.vk_app.as_mut().unwrap().render(self.window.as_ref().unwrap()) }.unwrap()
      }
      _ => (),
    }
  }
}

fn main() -> Result<()> {
  pretty_env_logger::init();

  // win

  let event_loop = EventLoop::new().unwrap();
  event_loop.set_control_flow(ControlFlow::Poll);

  event_loop.set_control_flow(ControlFlow::Wait);

  let mut app = App::default();
  event_loop
    .run_app(&mut app)
    .expect("Error while running Sagitario Engine");

  Ok(())
}
