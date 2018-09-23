use glium::{ Display, glutin };
use glutin::{ EventsLoop, ContextBuilder, WindowBuilder, GlWindow };
use imgui::{ ImGui };
use imgui_glium_renderer::{ Renderer };
use std::cell::Ref;


pub struct Conf {
    vsync: bool,
    title: String,
    width: f64,
    height: f64,
}

// Config is used to specify window setup
impl Conf {
    fn new(vsync: bool, title: String, height: f64, width: f64) -> Self {
        Self {
            vsync,
            title,
            width,
            height,
        }
    }
}

// Window  an object that holds on to global resources
pub struct Window {
    context: EventsLoop,
    //window: Ref<'a, GlWindow>,
    display: Display,
    imgui: ImGui,
    hidpi_factor: f64,
    renderer: Renderer,
}

impl Window {
    fn new(conf: Conf) -> Self {
        let context = ContextBuilder::new()
            .with_vsync(conf.vsync);
        let window = WindowBuilder::new()
            .with_title(conf.title)
            .with_dimensions(glutin::dpi::LogicalSize::new(conf.width, conf.height));

        Self {

        }
    }
}