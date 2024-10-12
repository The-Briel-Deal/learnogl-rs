use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::{Cursor, Window},
};

use learn_ogl_rs::state::App;

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
