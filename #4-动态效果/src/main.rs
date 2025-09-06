use std::{f32, time::SystemTime};

use animation::{Drawable, MyWindow};
use glium::{
    Display, DrawParameters, Surface as _, Texture2d,
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    implement_vertex,
    texture::RawImage2d,
    uniforms::Sampler,
    winit::{event_loop::EventLoopBuilder, window::Window},
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
            include_str!("./shaders/box.vert"),
            include_str!("./shaders/box.frag"),
            None,
        )
        .unwrap();

        let time = SystemTime::now()
            .duration_since(self.time)
            .unwrap()
            .as_secs_f32()
            * 30.0;
        let transform = mats::rotate3(radian(time), [1.0, 1.0, 1.0].into());
        let view = mats::translate3([0.0, 0.0, -5.0].into());
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
            texture,
            time: SystemTime::now(),
        },
        window,
        display,
    );
    event_loop.run_app(&mut app).unwrap();
}
