#![allow(dead_code, unused_variables, clippy::too_many_arguments, clippy::unnecessary_wraps)]
use anyhow::Result;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Theme, Window, WindowId};

#[derive(Default)]
struct App {
  window: Option<Window>,
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let custom_window = Window::default_attributes()
      .with_theme(Some(Theme::Dark))
      .with_title("Sagitario Engine")
      .with_active(true);
    self.window = Some(event_loop.create_window(custom_window).unwrap());
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        print!("The close button was pressed, stopping");
        event_loop.exit();
      }
      WindowEvent::RedrawRequested => {
        self.window.as_ref().unwrap().request_redraw();
      }
      _ => (),
    }
  }
}

fn main() {
  println!("Starting window!");

  // win

  let event_loop = EventLoop::new().unwrap();
  event_loop.set_control_flow(ControlFlow::Poll);

  event_loop.set_control_flow(ControlFlow::Wait);

  let mut app = App::default();
  event_loop
    .run_app(&mut app)
    .expect("Error while running Sagitario Engine");
}
