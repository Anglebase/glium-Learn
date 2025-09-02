use glium::{
    Display, Surface as _,
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    winit::{event_loop::EventLoopBuilder, window::Window},
};
use mats::radian;
use transform::{Drawable, MyWindow, Vertex};

struct Canvas {}

impl Drawable for Canvas {
    fn draw(&mut self, window: &Window, display: &Display<WindowSurface>) {
        let mut target = display.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0);

        const VERTEX: [Vertex; 4] = [
            Vertex::new(0.5, -0.5),
            Vertex::new(-0.5, -0.5),
            Vertex::new(0.5, 0.5),
            Vertex::new(-0.5, 0.5),
        ];
        const INDICES: [u16; 6] = [0, 1, 2, 1, 2, 3];
        let vertex_buffer = glium::VertexBuffer::new(display, &VERTEX).unwrap();
        let indices = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &INDICES,
        )
        .unwrap();
        let program = glium::Program::from_source(
            display,
            include_str!("../shaders/shader.vert"),
            include_str!("../shaders/shader.frag"),
            None,
        )
        .unwrap();

        let transform = mats::rotate3(radian(45.0), [1.0, 1.0, 1.0].into());
        let view = mats::translate3([0.0, 0.0, -2.0].into());
        let pre = mats::perspective(
            45.0,
            window.inner_size().width as f32 / window.inner_size().height as f32,
            0.1,
            100.0,
        );
        let transform = pre * view * transform;

        let uniforms = glium::uniform! {
            transform: transform,
        };
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        target.finish().unwrap();
    }
}

fn main() {
    let event_loop = EventLoopBuilder::<()>::default().build().unwrap();
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);
    let mut app = MyWindow::new(Canvas {}, window, display);
    event_loop.run_app(&mut app).unwrap();
}
