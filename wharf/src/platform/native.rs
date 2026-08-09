use crate::*;
use winit::{
    self,
    application::ApplicationHandler,
    event::{MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

#[derive(Debug)]
struct WinitApplication<A: Application> {
    engine: Engine<A>,
    window: Option<Window>,
}

impl<A: Application> ApplicationHandler for WinitApplication<A> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .expect("Could not create window"),
        )
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }

        if let Some(wharf_event) = to_wharf_event(event) {
            self.engine.on_event(wharf_event);
        }
    }
}

pub fn run<A: Application>(engine: Engine<A>) {
    info!("Running using native backend (winit)");

    let event_loop = EventLoop::new().expect("Could not create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut winit_application = WinitApplication {
        engine,
        window: None,
    };

    event_loop
        .run_app(&mut winit_application)
        .expect("Could not run winit application");
}

fn to_wharf_event(win_event: WindowEvent) -> Option<Event> {
    match win_event {
        // Window
        WindowEvent::CloseRequested => Some(Event::WindowClose),
        WindowEvent::Resized(physical_size) => Some(Event::WindowResize {
            width: physical_size.width,
            height: physical_size.height,
        }),
        WindowEvent::Focused(focused) => match focused {
            true => Some(Event::WindowFocus),
            false => Some(Event::WindowLostFocus),
        },
        WindowEvent::Moved(physical_position) => Some(Event::WindowMoved {
            x: physical_position.x,
            y: physical_position.y,
        }),
        // App
        // Key
        WindowEvent::KeyboardInput {
            device_id: _,
            event,
            is_synthetic: _,
        } => {
            // Need to fix key handling properly later! Doesn't account for like any edge cases lol
            let key = match event.physical_key {
                winit::keyboard::PhysicalKey::Code(key_code) => key_code as u8,
                winit::keyboard::PhysicalKey::Unidentified(_native_key_code) => 0,
            };
            match event.state {
                winit::event::ElementState::Pressed => Some(Event::KeyPressed {
                    key_code: key,
                    is_repeat: event.repeat,
                }),
                winit::event::ElementState::Released => Some(Event::KeyReleased { key_code: key }),
            }
        }
        // Mouse
        WindowEvent::MouseInput {
            device_id: _,
            state,
            button,
        } => {
            let button_code = mouse_button_to_code(button);
            match state {
                winit::event::ElementState::Pressed => Some(Event::MouseButtonPressed {
                    mouse_code: button_code,
                }),
                winit::event::ElementState::Released => Some(Event::MouseButtonReleased {
                    mouse_code: button_code,
                }),
            }
        }
        WindowEvent::CursorMoved {
            device_id: _,
            position,
        } => Some(Event::MouseMoved {
            x: position.x,
            y: position.y,
        }),
        WindowEvent::MouseWheel {
            device_id: _,
            delta,
            phase: _,
        } => match delta {
            winit::event::MouseScrollDelta::LineDelta(x, y) => Some(Event::MouseScrolled {
                x_offset: x as f64,
                y_offset: y as f64,
            }),
            winit::event::MouseScrollDelta::PixelDelta(physical_position) => {
                Some(Event::MouseScrolled {
                    x_offset: physical_position.x,
                    y_offset: physical_position.y,
                })
            }
        },
        _ => None,
    }
}

fn mouse_button_to_code(button: MouseButton) -> u8 {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Back => 3,
        MouseButton::Forward => 4,
        MouseButton::Other(_) => 5,
    }
}
