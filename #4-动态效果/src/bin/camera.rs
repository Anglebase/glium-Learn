use std::{f32, time::SystemTime};

use animation::{Drawable, MyWindow};
use device_query::{DeviceQuery, DeviceState};
use glium::{
    Display, DrawParameters, Surface as _, Texture2d,
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    implement_vertex,
    texture::RawImage2d,
    uniforms::Sampler,
    winit::{
        dpi::PhysicalPosition,
        event::{KeyEvent, WindowEvent},
        event_loop::EventLoopBuilder,
        keyboard::{KeyCode, PhysicalKey},
        window::Window,
    },
};
use image::GenericImageView;
use mats::radian;

#[derive(Clone, Copy)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    tex_coord: (f32, f32),
}

#[derive(Default)]
struct Camera {
    position: mats::Vec3<f32>,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    fn new() -> Self {
        Self::default()
    }
}

impl Camera {
    fn view(&self) -> mats::Mat4<f32> {
        let direction = mats::Vec4::from([0.0, 0.0, -1.0, 0.0]);
        let direction = direction * mats::rotate3_x(mats::radian(self.pitch));
        let direction = direction * mats::rotate3_y(mats::radian(self.yaw));
        let target = self.position + direction.xyz();
        mats::look_at(self.position, target, [0.0, 1.0, 0.0].into())
    }

    fn handle(&mut self, epsilon: f32) {
        let direction = mats::Vec4::from([0.0, 0.0, -1.0, 0.0]);
        let direction = direction * mats::rotate3_y(mats::radian(self.yaw));
        let delta_ws = direction.xyz() * epsilon;
        let delta_ad = (direction * mats::rotate3_y(radian(90.0))).xyz() * epsilon;

        let state = DeviceState::new();
        let keys = state.get_keys();
        if keys.contains(&device_query::Keycode::W) {
            self.position += delta_ws;
        }
        if keys.contains(&device_query::Keycode::S) {
            self.position -= delta_ws;
        }
        if keys.contains(&device_query::Keycode::A) {
            self.position -= delta_ad;
        }
        if keys.contains(&device_query::Keycode::D) {
            self.position += delta_ad;
        }
        if keys.contains(&device_query::Keycode::Space) {
            self.position[0][1] += epsilon;
        }
        if keys.contains(&device_query::Keycode::LShift) {
            self.position[0][1] -= epsilon;
        }
    }
}

impl Vertex {
    const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            tex_coord: (0.0, 0.0),
        }
    }
}

implement_vertex!(Vertex, x, y, z, tex_coord);

struct Canvas {
    texture: Texture2d,
    time: SystemTime,
    dt: SystemTime,
    camera: Camera,

    cursor_lock: bool,
}

impl Drawable for Canvas {
    fn draw(&mut self, window: &Window, display: &Display<WindowSurface>) {
        let mut target = display.draw();
        target.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), f32::INFINITY);
        const VERTEX: [Vertex; 8] = [
            Vertex::new(1.0, 1.0, 1.0),
            Vertex::new(1.0, 1.0, -1.0),
            Vertex::new(1.0, -1.0, 1.0),
            Vertex::new(1.0, -1.0, -1.0),
            Vertex::new(-1.0, 1.0, 1.0),
            Vertex::new(-1.0, 1.0, -1.0),
            Vertex::new(-1.0, -1.0, 1.0),
            Vertex::new(-1.0, -1.0, -1.0),
        ];
        const INDICES: [u16; 36] = [
            4, 2, 0, 4, 6, 2, // front
            1, 7, 5, 1, 3, 7, // back
            0, 3, 1, 0, 2, 3, // right
            5, 6, 4, 5, 7, 6, // left
            5, 0, 1, 5, 4, 0, // top
            2, 7, 6, 2, 3, 7, // bottom
        ];

        let vertex = INDICES
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let mut v = VERTEX[item as usize];
                match i % 6 {
                    0 => v.tex_coord = (0.0, 0.0),
                    1 => v.tex_coord = (1.0, 1.0),
                    2 => v.tex_coord = (1.0, 0.0),
                    3 => v.tex_coord = (0.0, 0.0),
                    4 => v.tex_coord = (0.0, 1.0),
                    5 => v.tex_coord = (1.0, 1.0),
                    _ => unreachable!(),
                }
                v
            })
            .collect::<Vec<_>>();

        let vertex_buffer = glium::VertexBuffer::new(display, &vertex).unwrap();
        let indices = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &(0..36).into_iter().collect::<Vec<u16>>(),
        )
        .unwrap();

        let program = glium::Program::from_source(
            display,
            include_str!("../shaders/box.vert"),
            include_str!("../shaders/box.frag"),
            None,
        )
        .unwrap();

        let time = SystemTime::now()
            .duration_since(self.time)
            .unwrap()
            .as_secs_f32()
            * 30.0;
        let transform = mats::rotate3(radian(time), [1.0, 1.0, 1.0].into());
        let view = self.camera.view();
        let pre = mats::perspective(
            45.0,
            window.inner_size().width as f32 / window.inner_size().height as f32,
            0.1,
            100.0,
        );
        let transform = pre * view * transform;

        let tex = Sampler::new(&self.texture);

        let uniforms = glium::uniform! {
            transform: transform,
            tex: tex,
        };

        let mut param = DrawParameters::default();
        param.depth.write = true;
        param.depth.test = glium::DepthTest::IfLess;
        target
            .draw(&vertex_buffer, &indices, &program, &uniforms, &param)
            .unwrap();

        target.finish().unwrap();
        self.camera
            .handle(self.dt.elapsed().unwrap().as_secs_f32() * 2.0);
        self.dt = SystemTime::now();
    }

    fn handle(
        &mut self,
        window: &Window,
        event_loop: &glium::winit::event_loop::ActiveEventLoop,
        _window_id: glium::winit::window::WindowId,
        event: glium::winit::event::WindowEvent,
    ) {
        let state = DeviceState::new();
        let keys = state.get_keys();
        let old = self.cursor_lock;
        self.cursor_lock = !keys.contains(&device_query::Keycode::LAlt);
        if self.cursor_lock != old {
            if self.cursor_lock {
                window.set_cursor_visible(false);
            } else {
                window.set_cursor_visible(true);
            }
        }
        let cx = window.inner_size().width as i32 / 2;
        let cy = window.inner_size().height as i32 / 2;
        if self.cursor_lock {
            if let WindowEvent::CursorMoved {
                position: PhysicalPosition { x, y },
                ..
            } = event
            {
                let coords = (x as i32, y as i32);
                let epsilon = 0.05;
                let (x, y) = coords;
                let (dx, dy) = (x - cx, y - cy);
                self.camera.yaw += dx as f32 * epsilon;
                self.camera.pitch += dy as f32 * epsilon;
                self.camera.pitch = self.camera.pitch.max(-89.9).min(89.9);
                self.camera.yaw = self.camera.yaw.rem_euclid(360.0);
            }
        }
        if self.cursor_lock {
            window
                .set_cursor_position(PhysicalPosition { x: cx, y: cy })
                .unwrap();
        }
        if let WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },
            ..
        } = event
        {
            event_loop.exit();
        }
    }
}

fn main() {
    let event_loop = EventLoopBuilder::<()>::default().build().unwrap();

    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);
    let image = image::ImageReader::open("./textures/石墙纹理.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let dimensions = image.dimensions();
    let image = image.to_rgba8().into_vec();
    let texture = Texture2d::new(&display, RawImage2d::from_raw_rgba(image, dimensions)).unwrap();

    let mut app = MyWindow::new(
        Canvas {
            camera: Camera::new(),
            texture,
            time: SystemTime::now(),
            dt: SystemTime::now(),
            cursor_lock: false,
        },
        window,
        display,
    );
    event_loop.run_app(&mut app).unwrap();
}
