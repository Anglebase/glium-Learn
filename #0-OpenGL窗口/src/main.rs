use glium::{
    Display, Surface,
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    winit::{
        application::ApplicationHandler, event::WindowEvent, event_loop::EventLoopBuilder,
        window::Window,
    },
};

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
