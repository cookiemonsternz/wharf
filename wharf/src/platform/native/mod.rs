use crate::wharf::{Application, engine::Engine};
use crate::*;
use winit::event::WindowEvent;
use winit::window::Window;
use winit::{
    self,
    application::ApplicationHandler,
    event_loop::{ControlFlow, EventLoop},
};

#[derive(Debug, Default)]
struct WinitApplication {
    window: Option<Window>,
}

impl ApplicationHandler for WinitApplication {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .expect("Could not create window"),
        )
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) => info!("{}, {}", size.width, size.height),
            WindowEvent::RedrawRequested => {
                info!("Redrawing");
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}

pub fn run<A: Application>(engine: Engine<A>) {
    info!("Running using native backend (winit)");

    let event_loop = EventLoop::new().expect("Could not create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut winit_application = WinitApplication {
        ..Default::default()
    };
    event_loop
        .run_app(&mut winit_application)
        .expect("Could not run winit application");
}
