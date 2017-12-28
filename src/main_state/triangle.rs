use cgmath::Vector2;
use cgmath::vec2;

pub type Triangle = [Vector2<f64>; 3];

pub struct TriangleNode {
    pub verts: Triangle,
    pub progress: f32,
}

impl TriangleNode {

    //Constructor
    pub fn new(tri: Triangle, progress: f32) -> TriangleNode {
        TriangleNode {
            verts: tri,
            progress,
        }
    }

    //Create TriangleNode from vector or slice.
    //Panics if length of the vector or slice is not 3.
    pub fn from_vec(tri: &[Vector2<f64>], progress: f32) -> TriangleNode {

        assert_eq!(tri.len(), 3);

        let mut triangle = [vec2(0.0, 0.0); 3];

        //Put the vertices into an array
        triangle.clone_from_slice(&tri[..]);

        TriangleNode {
            verts: triangle,
            progress,
        }
    }
}
