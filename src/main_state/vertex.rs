use cgmath::Vector2;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub time: f32,
}

implement_vertex!(Vertex, position, time);

impl Vertex {

    //Constructor
    pub fn new(position : [f32; 2], time: f32) -> Vertex {
        Vertex { position, time }
    }

    //Creates a Vertex from a Vector2<f64>
    pub fn from_vector2(v : &Vector2<f64>, time: f32) -> Vertex {
        Vertex { position: [v.x as f32, v.y as f32], time }
    }
}