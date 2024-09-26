use core::panic;
use std::{cell::RefCell, num::NonZeroU32, ops::Deref, rc::Rc};

use glutin::{
    config::{Config, ConfigTemplateBuilder, GetGlConfig, GlConfig},
    context::{
        ContextApi, ContextAttributesBuilder, NotCurrentContext, PossiblyCurrentContext, Version,
    },
    display::GetGlDisplay,
    prelude::{GlDisplay, NotCurrentGlContext},
};
use glutin_winit::DisplayBuilder;
use softbuffer::Surface;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    raw_window_handle::HasWindowHandle,
    window::{self, Window},
};

type RcSurf = Rc<RefCell<Surface<Rc<Window>, Rc<Window>>>>;
struct App {
    window: Option<Rc<Window>>,
    surface: Option<RcSurf>,
    template: ConfigTemplateBuilder,
    gl_display: GlDisplayCreationState,
    gl_context: Option<PossiblyCurrentContext>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let (window, gl_config) = match &self.gl_display {
            GlDisplayCreationState::Builder(display_builder) => {
                let (window, gl_config) = match display_builder.clone().build(
                    event_loop,
                    self.template.clone(),
                    |configs| {
                        configs
                            .reduce(|accum, config| {
                                dbg!(&config);
                                if config.num_samples() > accum.num_samples() {
                                    config
                                } else {
                                    accum
                                }
                            })
                            .unwrap()
                    },
                ) {
                    Ok((window, gl_config)) => (window.unwrap(), gl_config),
                    Err(err) => {
                        panic!("Builder returned error {err}")
                    }
                };
                self.gl_display = GlDisplayCreationState::Init;

                self.gl_context =
                    Some(create_gl_context(&window, &gl_config).treat_as_possibly_current());
                (window, gl_config)
            }
            GlDisplayCreationState::Init => {
                let gl_config = self.gl_context.as_ref().unwrap().config();

                match glutin_winit::finalize_window(
                    event_loop,
                    Window::default_attributes()
                        .with_transparent(true)
                        .with_title("Glutin triangle gradient example (press Escape to exit)"),
                    &gl_config,
                ) {
                    Ok(window) => (window, gl_config),
                    Err(err) => panic!("Window finalization failed {err}"),
                }
            }
        };
        // self.window = Some(Rc::new(
        //     event_loop
        //         .create_window(Window::default_attributes())
        //         .unwrap(),
        // ));
        // let context = softbuffer::Context::new(self.window.clone().unwrap()).unwrap();
        // self.surface = Some(Rc::new(RefCell::new(
        //     softbuffer::Surface::new(&context, self.window.clone().unwrap()).unwrap(),
        // )));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        dbg!(&event);
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let surface = self.surface.clone().unwrap();
                let mut surface = surface.borrow_mut();

                let buffer = surface.buffer_mut();
                let mut buffer = buffer.unwrap();

                for pixel in buffer.iter_mut() {
                    *pixel = u32::MIN;
                }
                let _ = buffer.present();
                // self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::Resized(size) => {
                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    self.surface
                        .clone()
                        .unwrap()
                        .borrow_mut()
                        .resize(width, height)
                        .unwrap();
                }
            }
            _ => (),
        }
    }
}

enum GlDisplayCreationState {
    /// The display was not build yet.
    Builder(DisplayBuilder),
    /// The display was already created for the application.
    Init,
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let template = ConfigTemplateBuilder::new();

    let display_builder = DisplayBuilder::new().with_window_attributes(Some(
        Window::default_attributes()
            .with_transparent(true)
            .with_title("Glutin triangle gradient example (press Escape to exit)"),
    ));

    let mut app = App {
        template,
        gl_display: GlDisplayCreationState::Builder(display_builder),
        window: None,
        surface: None,
        gl_context: None,
    };
    let _ = event_loop.run_app(&mut app);
}

fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

    // The context creation part.
    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    // There are also some old devices that support neither modern OpenGL nor GLES.
    // To support these we can try and create a 2.1 context.
    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(raw_window_handle);

    // Reuse the uncurrented context from a suspended() call if it exists, otherwise
    // this is the first time resumed() is called, where the context still
    // has to be created.
    let gl_display = gl_config.display();

    unsafe {
        gl_display
            .create_context(gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(gl_config, &fallback_context_attributes)
                    .unwrap_or_else(|_| {
                        gl_display
                            .create_context(gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    })
            })
    }
}
