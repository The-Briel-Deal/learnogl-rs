use core::panic;
use std::{
    cell::RefCell,
    num::NonZeroU32,
    ops::Deref,
    rc::Rc,
};

use glutin::{
    config::{Config, ConfigTemplateBuilder, GetGlConfig, GlConfig},
    context::{
        ContextApi, ContextAttributesBuilder, NotCurrentContext, PossiblyCurrentContext, Version,
    },
    display::GetGlDisplay,
    prelude::{GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext},
    surface::{GlSurface, Surface, SwapInterval, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    raw_window_handle::HasWindowHandle,
    window::Window,
};
use winit_test::renderer::Renderer;

type RcSurf = Rc<RefCell<Surface<WindowSurface>>>;

struct App {
    window: Option<Rc<Window>>,
    surface: Option<RcSurf>,
    template: ConfigTemplateBuilder,
    gl_display: GlDisplayCreationState,
    gl_context: Option<PossiblyCurrentContext>,
    renderer: Option<Renderer>,
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

        let attrs = window
            .build_surface_attributes(Default::default())
            .expect("Failed to build surface attributes");
        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let gl_context = self.gl_context.as_ref().unwrap();

        gl_context.make_current(&gl_surface).unwrap();

        self.renderer
            .get_or_insert_with(|| Renderer::new(&gl_config.display()));

        // Try setting vsync.
        if let Err(res) = gl_surface
            .set_swap_interval(gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            eprintln!("Error setting vsync: {res:?}");
        }
        self.surface = Some(Rc::new(RefCell::new(gl_surface)));
        self.window = Some(Rc::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        dbg!(&event);
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                let gl_surface = self.surface.as_ref().unwrap();
                let gl_surface = gl_surface.deref().borrow();

                let gl_context = self.gl_context.as_ref().unwrap();
                let renderer = self.renderer.as_ref().unwrap();
                renderer.draw();
                window.request_redraw();

                gl_surface.swap_buffers(gl_context).unwrap();
            }
            WindowEvent::Resized(size) => {
                if let Some(gl_surface) = self.surface.clone() {
                    let gl_context = self.gl_context.as_ref().unwrap();

                    let gl_surface = gl_surface.deref().borrow();
                    gl_surface.resize(
                        gl_context,
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    );

                    let renderer = self.renderer.as_ref().unwrap();
                    renderer.resize(size.width as i32, size.height as i32);
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
        renderer: None,
    };
    let _ = event_loop.run_app(&mut app);
}

fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(raw_window_handle);

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
