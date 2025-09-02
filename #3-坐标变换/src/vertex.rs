use glium::implement_vertex;

#[derive(Clone, Copy)]
pub struct Vertex {
    x: f32,
    y: f32,
}

implement_vertex!(Vertex, x, y);

impl Vertex {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}