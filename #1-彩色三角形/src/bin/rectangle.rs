use glium::{
    Display, Surface,
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    implement_vertex,
    winit::{
        application::ApplicationHandler, event::WindowEvent, event_loop::EventLoopBuilder,
        window::Window,
    },
};

#[derive(Clone, Copy)]
struct Vertex {
    x: f32,
    y: f32,
    color: [f32; 3],
}

implement_vertex!(Vertex, x, y, color);

impl Vertex {
    const fn new(x: f32, y: f32, color: [f32; 3]) -> Self {
        Self { x, y, color }
    }
}

#[allow(unused)]
struct MyWindow {
    window: Window,
    display: Display<WindowSurface>,
}

impl MyWindow {
    fn new(window: Window, display: Display<WindowSurface>) -> Self {
        Self { window, display }
    }

    fn draw(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0);

        const VERTEX: [Vertex; 4] = [
            Vertex::new(0.5, -0.5, [1.0, 0.0, 0.0]),
            Vertex::new(-0.5, -0.5, [0.0, 1.0, 0.0]),
            Vertex::new(0.5, 0.5, [0.0, 0.0, 1.0]),
            Vertex::new(-0.5, 0.5, [1.0, 1.0, 0.0]),
        ];
        const INDICES: [u16; 6] = [0, 1, 2, 3, 2, 1];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &VERTEX).unwrap();
        let indices = glium::IndexBuffer::new(
            &self.display,
            glium::index::PrimitiveType::TrianglesList,
            &INDICES,
        )
        .unwrap();
        let program = glium::Program::from_source(
            &self.display,
            "
            #version 330

            in float x;
            in float y;
            in vec3 color;

            out vec3 v_color;

            void main() {
                v_color = color;
                gl_Position = vec4(x, y, 0.0, 1.0);
            }
            ",
            "
            #version 330

            in vec3 v_color;

            out vec4 color;

            void main() {
                color = vec4(v_color, 1.0);
            }
            ",
            None,
        )
        .unwrap();
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
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
    let mut app = MyWindow::new(window, display);
    event_loop.run_app(&mut app).unwrap();
}
