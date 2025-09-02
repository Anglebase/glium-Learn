use glium::{
    Display, Surface, Texture2d,
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    implement_vertex,
    texture::RawImage2d,
    uniform,
    uniforms::Sampler,
    winit::{
        application::ApplicationHandler, event::WindowEvent, event_loop::EventLoopBuilder,
        window::Window,
    },
};
use image::GenericImageView;

#[derive(Clone, Copy)]
struct Vertex {
    x: f32,
    y: f32,
    in_tex_coord: [f32; 2],
}

implement_vertex!(Vertex, x, y, in_tex_coord);

impl Vertex {
    const fn new(x: f32, y: f32, tex: [f32; 2]) -> Self {
        Self {
            x,
            y,
            in_tex_coord: tex,
        }
    }
}

#[allow(unused)]
struct MyWindow {
    window: Window,
    display: Display<WindowSurface>,
    texture: Texture2d,
}

impl MyWindow {
    fn new(window: Window, display: Display<WindowSurface>, texture: Texture2d) -> Self {
        Self {
            window,
            display,
            texture,
        }
    }

    fn draw(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0);

        const VERTEX: [Vertex; 4] = [
            Vertex::new(0.0, 0.5, [0.5, 0.0]),
            Vertex::new(0.5, 0.0, [1.0, 0.5]),
            Vertex::new(0.0, -0.5, [0.5, 1.0]),
            Vertex::new(-0.5, 0.0, [0.0, 0.5]),
        ];
        const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &VERTEX).unwrap();
        let indices = glium::IndexBuffer::new(
            &self.display,
            glium::index::PrimitiveType::TrianglesList,
            &INDICES,
        )
        .unwrap();

        let program = glium::Program::from_source(
            &self.display,
            include_str!("../shaders/texture.vert"),
            include_str!("../shaders/texture.frag"),
            None,
        )
        .unwrap();
        let tex = Sampler::new(&self.texture);
        let uniforms = uniform! {
            tex: tex,
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

impl ApplicationHandler for MyWindow {
    fn resumed(&mut self, _event_loop: &glium::winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &glium::winit::event_loop::ActiveEventLoop,
        _window_id: glium::winit::window::WindowId,
        event: glium::winit::event::WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }

        if let WindowEvent::RedrawRequested = event {
            self.draw();
        }
    }
}

fn main() {
    let event_loop = EventLoopBuilder::<()>::default().build().unwrap();
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

    let image = image::ImageReader::open("./textures/木质纹理.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let dimensions = image.dimensions();
    let image = image.to_rgba8().into_vec();
    let texture = Texture2d::new(&display, RawImage2d::from_raw_rgba(image, dimensions)).unwrap();

    let mut app = MyWindow::new(window, display, texture);
    event_loop.run_app(&mut app).unwrap();
}
