use glium::{
    Display,
    glutin::surface::WindowSurface,
    winit::{application::ApplicationHandler, event::WindowEvent, window::Window},
};

pub trait Drawable {
    fn draw(&mut self, window: &Window, display: &Display<WindowSurface>);
}

pub struct MyWindow<T: Drawable> {
    impl_: T,
    window: Window,
    display: Display<WindowSurface>,
}

impl<T: Drawable> MyWindow<T> {
    pub fn new(impl_: T, window: Window, display: Display<WindowSurface>) -> Self {
        Self {
            impl_,
            window,
            display,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn display(&self) -> &Display<WindowSurface> {
        &self.display
    }
}

impl<T: Drawable> ApplicationHandler for MyWindow<T> {
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
            self.impl_.draw(&self.window, &self.display);
        }
    }
}
