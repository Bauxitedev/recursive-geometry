
use main_state::TriangleNode;

use std::f64::consts::*;
use cgmath::vec2;
use cgmath::Vector2;

//An enum describing the starting configuration of the program.
#[derive(Debug, Copy, Clone)]
pub enum Configuration {
    Triangle, //Triangle
    NGon(usize), //Triangle fan with a certain amount of vertices
    Square //Two triangles forming a square
}

impl Configuration {

    pub fn get_triangles(self) -> Vec<TriangleNode> {

        let size = 0.95; //1.0 exactly hits the edge of the screen, so this gives a little padding

        match self {

            Configuration::Triangle => {

                //Generate a single triangle, aligned in the center
                let trio_vec:Vec<Vector2<f64>> = [0, 1, 2].iter().map(|&i| {
                    let (y, x) = (f64::from(i) * FRAC_PI_3 * 2.0).sin_cos();
                    vec2(x, y)
                }).collect();

                vec!(TriangleNode::from_vec(&trio_vec, 0.0))

            }

            Configuration::NGon(n) => {

                //Any ngon with n < 3 is degenerate
                assert!(n >= 3);

                //Generate points in a n-gon, aligned in the center
                let point_vec: Vec<_> = (0..n).map(|i| {
                    let (y, x) = (i as f64 * PI / n as f64 * 2.0).sin_cos();
                    vec2(x, y) * size
                }).collect();

                //Generate triangles from these points, in the form of a triangle fan
                let center = vec2(0.0, 0.0);
                point_vec.iter().cycle().take(n+1).collect::<Vec<_>>().windows(2).map(|edge| {
                    TriangleNode::from_vec(&[*edge[0], *edge[1], center], 0.0)
                }).collect()

            }

            Configuration::Square => {

                //Generate a square using two halves
                let a = TriangleNode::from_vec(&[vec2(-size, -size), vec2(size, -size),  vec2(-size, size)], 0.0);
                let b = TriangleNode::from_vec(&[vec2(size,   size), vec2(-size, size),  vec2(size, -size)], 0.0);

                vec!(a, b)

            }

        }

    }
}