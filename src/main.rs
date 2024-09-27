use core::panic;
use std::{num::NonZeroU32, rc::Rc};

use glutin::{
    config::{Config, ConfigTemplateBuilder, GetGlConfig, GlConfig},
    context::PossiblyCurrentContext,
    display::GetGlDisplay,
    prelude::{GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext},
    surface::{GlSurface, Surface, SwapInterval, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use winit_test::{gl::create_gl_context, renderer::Renderer, state::App};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let template = ConfigTemplateBuilder::new();
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(
        Window::default_attributes()
            .with_transparent(true)
            .with_title("Glutin triangle gradient example (press Escape to exit)"),
    ));

    let mut app = App::new(template, display_builder);
    let _ = event_loop.run_app(&mut app);
}
