use std::fmt::{Debug, Display};

pub type KeyCode = u8;
pub type MouseCode = u8;

#[derive(Debug)]
pub enum Event {
    // Window
    WindowClose,
    WindowResize { width: u32, height: u32 },
    WindowFocus,
    WindowLostFocus,
    WindowMoved { x: u32, y: u32 },
    // App
    AppTick,
    AppUpdate,
    AppRender,
    // Key
    KeyPressed { key_code: KeyCode, is_repeat: bool },
    KeyReleased { key_code: KeyCode },
    // Mouse
    MouseButtonPressed { mouse_code: MouseCode },
    MouseButtonReleased { mouse_code: MouseCode },
    MouseMoved { x: f64, y: f64 },
    MouseScrolled { x_offset: f64, y_offset: f64 },
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::WindowClose => write!(f, "WindowCloseEvent"),
            Event::WindowResize { width, height } => {
                write!(f, "WindowResizeEvent: w:{} h:{}", width, height)
            }
            Event::WindowFocus => write!(f, "WindowFocusEvent"),
            Event::WindowLostFocus => write!(f, "WindowLostFocusEvent"),
            Event::WindowMoved { x, y } => write!(f, "WindowMovedEvent: x:{} y:{}", x, y),
            Event::AppTick => write!(f, "AppTickEvent"),
            Event::AppUpdate => write!(f, "AppUpdateEvent"),
            Event::AppRender => write!(f, "AppRenderEvent"),
            Event::KeyPressed {
                key_code,
                is_repeat,
            } => write!(
                f,
                "KeyPressedEvent, key_code:{} (is_repeat: {})",
                key_code, is_repeat
            ),
            Event::KeyReleased { key_code } => write!(f, "KeyReleasedEvent, key_code:{}", key_code),
            Event::MouseButtonPressed { mouse_code } => {
                write!(f, "MouseButtonPressedEvent, mouse_code:{}", mouse_code)
            }
            Event::MouseButtonReleased { mouse_code } => {
                write!(f, "MouseButtonReleasedEvent, mouse_code:{}", mouse_code)
            }
            Event::MouseMoved { x, y } => write!(f, "MouseMovedEvent: x:{} y:{}", x, y),
            Event::MouseScrolled { x_offset, y_offset } => write!(
                f,
                "MouseScrolledEvent: x_offset:{} y_offset:{}",
                x_offset, y_offset
            ),
        }
    }
}
