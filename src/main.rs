use glium::draw_parameters::PolygonMode;
use glium::glutin;
use glium::glutin::event::VirtualKeyCode;
use glium::glutin::window::Fullscreen;
use glium::uniform;
use glium::Surface;
use glutin::event::ElementState;
use glutin::window::CursorGrabMode;
use std::collections::HashSet;
use glutin::event::Event::DeviceEvent;
use glutin::event::DeviceEvent::MouseMotion;

use camera::Camera;
use chunk::Chunk;
use std::collections::HashMap;
use std::io::Cursor;

mod camera;
mod chunk;
mod cube;
mod perspective;
mod vertex;
mod view_matrix;

struct Object {}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Raft");
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let monitor_handle = display
        .gl_window()
        .window()
        .available_monitors()
        .next()
        .unwrap();
    let fs = Fullscreen::Borderless(Some(monitor_handle));
    display.gl_window().window().set_fullscreen(Some(fs));

    display.gl_window().window().set_cursor_visible(false);
    display
        .gl_window()
        .window()
        .set_cursor_grab(CursorGrabMode::Locked)
        .unwrap();

    let vertex_shader_src = include_str!("vertex.glsl");
    let fragment_shader_src = include_str!("fragment.glsl");

    let image = image::load(
        Cursor::new(&include_bytes!("diffuse.jpg")),
        image::ImageFormat::Jpeg,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let mut chunk: Chunk = Default::default();
    for i in 0..16 {
        for j in 0..16 {
            chunk.blocks[i][0][j] = 1;
        }
    }
    let (chunk_shape, chunk_indices) = chunk.prepare(&display);

    let mut block_types: HashMap<u8, Object> = HashMap::new();
    block_types.insert(1, Object {});

    let image = image::load(
        Cursor::new(&include_bytes!("normal.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let normal_map = glium::texture::Texture2d::new(&display, image).unwrap();

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut pressed_keys = HashSet::new();

    let mut camera = Camera::default();
    let mut polygon_lines = false;

    event_loop.run(move|ev, _, control_flow| {
        let mut target = display.draw();

        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        camera.handle_keys(&pressed_keys, &chunk);

        if pressed_keys.contains(&VirtualKeyCode::Escape) {
            *control_flow = glutin::event_loop::ControlFlow::Exit; //TODO: why does this not work
            std::process::exit(0);
        }

        let perspective = perspective::create_perspective(&target);

        let light = [0.0, 1.0, 0.7f32];

        let direction = camera.get_direction();
        let view = view_matrix::view_matrix(&[camera.position.0, camera.position.1, camera.position.2], &[direction.0, direction.1, direction.2], &[0.0, 1.0, 0.0]);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            polygon_mode: if polygon_lines { PolygonMode::Line } else { PolygonMode::Fill },
            .. Default::default()
        };

        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];
        target.draw(&chunk_shape, &chunk_indices, &program, &uniform! { perspective: perspective, model: model, view: view, u_light: light, diffuse_tex: &diffuse_texture, normal_tex: &normal_map }, &params).unwrap();

        target.finish().unwrap();
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            DeviceEvent { event: MouseMotion { delta }, .. } => {
                camera.rotate(delta);
            },
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                },
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        if keycode == VirtualKeyCode::F1 {
                            polygon_lines = !polygon_lines;
                        }
                        if input.state == ElementState::Pressed {
                            pressed_keys.insert(keycode);
                        } else {
                            pressed_keys.remove(&keycode);
                        }
                    }
                },
                _ => (),
            },
            _ => (),
        }
    });
}

fn rotate(vec: [f32; 3], alpha: f32, beta: f32, gamma: f32) -> [f32; 3] {
    let mat: [[f32; 3]; 3] = [
        [
            beta.cos() * gamma.cos(),
            alpha.sin() * beta.sin() * gamma.cos() - alpha.cos() * gamma.sin(),
            alpha.cos() * beta.sin() * gamma.cos() + alpha.sin() * gamma.sin(),
        ],
        [
            beta.cos() * gamma.sin(),
            alpha.sin() * beta.sin() * gamma.sin() + alpha.cos() * gamma.cos(),
            alpha.cos() * beta.sin() * gamma.sin() - alpha.sin() * gamma.cos(),
        ],
        [
            -beta.sin(),
            alpha.sin() * beta.cos(),
            alpha.cos() * beta.cos(),
        ],
    ];
    [
        vec[0] * mat[0][0] + vec[1] * mat[0][1] + vec[2] * mat[0][2],
        vec[0] * mat[1][0] + vec[1] * mat[1][1] + vec[2] * mat[1][2],
        vec[0] * mat[2][0] + vec[1] * mat[2][1] + vec[2] * mat[2][2],
    ]
}
