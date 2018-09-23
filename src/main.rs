#[macro_use]
extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;

use glium::{ glutin, Surface };
use glium::uniforms::EmptyUniforms;
use imgui_glium_renderer::Renderer;
use imgui::*;
use std::time::Instant;


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new()
        .with_vsync(false);

    let display =  match glium::Display::new(window, context, &events_loop) {
        Ok(i) => i,
        Err(err) => panic!("Failed to create window: {:?}", err)
    };
    //let window = display.gl_window();

    let mut imgui = ImGui::init();
    let mut renderer = Renderer::init(&mut imgui, &display).unwrap();

    let mut last_frame = Instant::now();
    let mut rgb =  [0.0, 0.0, 0.0];

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    let vertex_positions = [ 
        Vertex { position: [0.0, -0.5] },
        Vertex { position: [0.5, 0.5] },
        Vertex { position: [-0.5, 0.5] }

    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertex_positions)
        .unwrap();
    
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        #extension GL_KHR_vulkan_glsl : enable

        in vec2 position;
        uniform float t;
        out vec2 fragColor;


        vec3 colors[3] = vec3[](
            vec3(1.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            vec3(0.0, 0.0, 1.0)
        );

        void main() {
            vec2 pos = position;
            pos.x += t;
            fragColor = position;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 fragColor;
        out vec4 f_color;

        void main() {
            f_color = vec4(fragColor, 1.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
        .expect("Failed to create program");
    

    let mut t: f32 = -0.5;
    let mut closed = false;
    while !closed {

        // we update `t`
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(rgb[0], rgb[1], rgb[2], 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! { t: t },
                    &Default::default()).expect("Failed to draw");


        let now = Instant::now();
	    let delta = now - last_frame;
	    let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        let fps = (1.0 / (delta.as_secs() as f64 + delta.subsec_nanos() as f64 * 1e-9)) as i32;
        last_frame = now;

        //let inner_size = window.get_inner_size().unwrap();
        //let hidpi_factor = window.get_hidpi_factor();
        //let frame_size = FrameSize::new(inner_size.width, inner_size.height, hidpi_factor);
        //let ui = imgui.frame(frame_size, delta_s);
        let ui = Ui::current_ui().unwrap();


        let mut color = EditableColor::Float3(&mut rgb);
        let color_edit = imgui::ColorEdit::new(&ui, im_str!("bg color"), color);
        // imgui ui
        ui.window(im_str!("Debug"))
            .position((10.0, 10.0), ImGuiCond::FirstUseEver)
            .size((200.0, 100.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.text(im_str!(""));
                ui.separator();
                ui.text(im_str!(
                    "FPS: ({})", fps,
                        ));
            });

        ui.window(im_str!("Background"))
            .position((250.0, 10.0), ImGuiCond::Appearing)
            .size((200.0, 100.0), ImGuiCond::Appearing)
            .build(|| {
                color_edit.build();
                if ui 
                    .button(im_str!("update"), (50.0, 20.0))
                {
                    println!("Clicked");
                }
        });
        

        renderer.render(&mut target, ui).unwrap();
        target.finish().unwrap();
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, ..} => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },

                _ => (),
            }
        });
    }
    
}
