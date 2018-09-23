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
use glium::glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase};
use glium::glutin::ElementState::Pressed;

mod window;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new()
        .with_vsync(false);
    let window = glutin::WindowBuilder::new()
        .with_title("Triangle")
        .with_dimensions(glutin::dpi::LogicalSize::new(600.0, 600.0));

    let display =  match glium::Display::new(window, context, &events_loop) {
        Ok(i) => i,
        Err(err) => panic!("Failed to create window: {:?}", err)
    };
    let window = display.gl_window();

    let mut imgui = ImGui::init();
    imgui.set_ini_filename(None);

    let hidpi_factor = window.get_hidpi_factor().round();

    let mut renderer = Renderer::init(&mut imgui, &display).unwrap();

    let mut last_frame = Instant::now();
    let mut mouse_state = MouseState::default();
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

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, ..} => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::CursorMoved { position: pos, .. } => {
                        // Rescale position from glutin logical coordinates to our logical
                        // coordinates
                        mouse_state.pos = pos
                            .to_physical(window.get_hidpi_factor())
                            .to_logical(hidpi_factor)
                            .into();
                    },
                    glutin::WindowEvent::MouseInput { state, button, .. } => match button {
                        MouseButton::Left => mouse_state.pressed.0 = state == Pressed,
                        MouseButton::Right => mouse_state.pressed.1 = state == Pressed,
                        MouseButton::Middle => mouse_state.pressed.2 = state == Pressed,
                        _ => {}
                    },
                    _ => (),
                },

                _ => (),
            }
        });

        let now = Instant::now();
	    let delta = now - last_frame;
	    let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        update_mouse(&mut imgui, &mut mouse_state);

        let mouse_cursor = imgui.mouse_cursor();
        if imgui.mouse_draw_cursor() || mouse_cursor == ImGuiMouseCursor::None {
            // Hide OS cursor
            window.hide_cursor(true);
        } else {
            // Set OS cursor
            window.hide_cursor(false);
            window.set_cursor(match mouse_cursor {
                ImGuiMouseCursor::None => unreachable!("mouse_cursor was None!"),
                ImGuiMouseCursor::Arrow => glutin::MouseCursor::Arrow,
                ImGuiMouseCursor::TextInput => glutin::MouseCursor::Text,
                ImGuiMouseCursor::Move => glutin::MouseCursor::Move,
                ImGuiMouseCursor::ResizeNS => glutin::MouseCursor::NsResize,
                ImGuiMouseCursor::ResizeEW => glutin::MouseCursor::EwResize,
                ImGuiMouseCursor::ResizeNESW => glutin::MouseCursor::NeswResize,
                ImGuiMouseCursor::ResizeNWSE => glutin::MouseCursor::NwseResize,
            });
        }

        // Rescale window size from glutin logical size to our logical size
        let physical_size = window
            .get_inner_size()
            .unwrap()
            .to_physical(window.get_hidpi_factor());
        let logical_size = physical_size.to_logical(hidpi_factor);

        let frame_size = FrameSize {
            logical_size: logical_size.into(),
            hidpi_factor,
        };

        let ui = imgui.frame(frame_size, delta_s);
   

        let mut color = EditableColor::Float3(&mut rgb);
        let color_edit = imgui::ColorEdit::new(&ui, im_str!("bg color"), color);
        // imgui ui
        ui.window(im_str!("Debug"))
            .position((10.0, 10.0), ImGuiCond::Appearing)
            .size((200.0, 100.0), ImGuiCond::Appearing)
            .build(|| {
                ui.text(im_str!(""));
                ui.separator();
                ui.text(im_str!(
                    "fps: ({})", ui.framerate() as u32,
                        ));              
            });


        ui.window(im_str!("Background"))
            .position((250.0, 10.0), ImGuiCond::Appearing)
            .size((200.0, 100.0), ImGuiCond::Appearing)
            .build(|| {
                color_edit.build();
                if ui 
                .color_button(im_str!("Green color"), (0.0, 1.0, 0.0, 1.0))
                .size((20.0, 20.0))
                .build()
                {

                }
            });
        renderer.render(&mut target, ui).unwrap();
        target.finish().unwrap();
    }
       
}

fn update_mouse(imgui: &mut ImGui, mouse_state: &mut MouseState) {
    imgui.set_mouse_pos(mouse_state.pos.0 as f32, mouse_state.pos.1 as f32);
    imgui.set_mouse_down([
        mouse_state.pressed.0,
        mouse_state.pressed.1,
        mouse_state.pressed.2,
        false,
        false,
    ]);
    imgui.set_mouse_wheel(mouse_state.wheel);
    mouse_state.wheel = 0.0;
}