use camera::Camera;
use glium::draw_parameters::PolygonMode;
use glium::glutin::event::VirtualKeyCode;
use glium::glutin::window::Fullscreen;
use glium::{glutin, uniform, Surface};
use glutin::event::DeviceEvent::MouseMotion;
use glutin::event::ElementState;
use glutin::event::Event::DeviceEvent;
use glutin::window::CursorGrabMode;
use math::Vec3;
use std::collections::HashSet;
use std::io::Cursor;
use world::World;

mod camera;
mod chunk;
mod cube;
mod math;
mod object;
mod perspective;
mod vertex;
mod view_matrix;
mod world;
mod world_gen;

use glium::implement_vertex;
#[derive(Copy, Clone)]
struct Vertex2D {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex2D, position, tex_coords);

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
    let fullscreen = false;
    if fullscreen {
        display.gl_window().window().set_fullscreen(Some(fs));
    }

    display.gl_window().window().set_cursor_visible(false);
    display
        .gl_window()
        .window()
        .set_cursor_grab(CursorGrabMode::Locked)
        .unwrap();

    let vertex_shader_src = include_str!("vertex.glsl");
    let fragment_shader_src = include_str!("fragment.glsl");

    let crosshair_vertex_src = include_str!("image_vertex.glsl");
    let crosshair_fragment_src = include_str!("image_fragment.glsl");

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

    let image = image::load(
        Cursor::new(&include_bytes!("crosshair.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let crosshair_tex = glium::texture::Texture2d::new(&display, image).unwrap();

    let crosshair_program =
        glium::Program::from_source(&display, crosshair_vertex_src, crosshair_fragment_src, None)
            .unwrap();

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut pressed_keys = HashSet::new();

    let mut camera = Camera::default();
    let mut polygon_lines = false;

    let mut world = World::new();
    world.generate_chunk(0, 0);
    camera.position = Vec3(8.0, 100.0, 8.0);
    //world.create_empty_chunk(0, 0);

    event_loop.run(move|ev, _, control_flow| {
        let mut target = display.draw();

        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        camera.handle_keys(&pressed_keys, &mut world);

        if pressed_keys.contains(&VirtualKeyCode::Escape) {
            *control_flow = glutin::event_loop::ControlFlow::Exit; //TODO: why does this not work
            std::process::exit(0);
        }

        let perspective = perspective::create_perspective(&target);

        let light = Vec3(1.4, -0.4, -0.7);

        let direction = camera.get_direction();
        let view = view_matrix::view_matrix(&[camera.position.0, camera.position.1, camera.position.2], &[direction.0, direction.1, direction.2], &[0.0, 1.0, 0.0]);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            blend: glium::Blend {
                color: glium::BlendingFunction::AlwaysReplace,
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One
                },
                constant_value: (1.0, 1.0, 1.0, 1.0)
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            polygon_mode: if polygon_lines { PolygonMode::Line } else { PolygonMode::Fill },
            .. Default::default()
        };

        for ((x, z), chunk) in world.chunks.iter_mut() {
            let model = [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [*x as f32 * 16.0, 0.0, *z as f32 * 16.0, 1.0f32]
            ];
            if chunk.mesh.is_none() {
                chunk.prepare(&display);
            }
            target.draw(&chunk.mesh.as_ref().unwrap().0, &chunk.mesh.as_ref().unwrap().1, &program, &uniform! { perspective: perspective, model: model, view: view, u_light: light.tuple(), diffuse_tex: &diffuse_texture, normal_tex: &normal_map }, &params).unwrap();
        }

        let crosshair_shape = vec![
            Vertex2D { position: [ -0.05, -0.05], tex_coords: [0.0, 0.0] },
            Vertex2D { position: [ -0.05, 0.05], tex_coords: [0.0, 1.0] },
            Vertex2D { position: [ 0.05, -0.05], tex_coords: [1.0, 0.0] },
            Vertex2D { position: [ 0.05, 0.05], tex_coords: [1.0, 1.0] },
            Vertex2D { position: [ 0.05, -0.05], tex_coords: [1.0, 0.0] },
            Vertex2D { position: [ -0.05, 0.05], tex_coords: [0.0, 1.0] },
        ];
        let crosshair_buffer = glium::VertexBuffer::new(&display, &crosshair_shape).unwrap();
        target.draw(&crosshair_buffer, glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), &crosshair_program, &uniform! {tex: &crosshair_tex}, &params).unwrap();

        target.finish().unwrap();
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            DeviceEvent { event: MouseMotion { delta }, .. } => {
                camera.rotate(delta);
            },
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::MouseInput { button, state, .. } => {
                    println!("handling click");
                    let mut vec = camera.position;
                    let direction = camera.get_direction();
                    loop {
                        vec += direction;
                        println!("{}", vec);
                        if let Some(block) = world.block_at(vec.0 as i32, vec.1 as i32, vec.2 as i32) {
                            println!("Hit {}, {}, {}", vec.0 as i32, vec.1 as i32, vec.2 as i32);
                            break;
                        }
                    }
                }
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
