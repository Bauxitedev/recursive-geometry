mod triangle;
mod vertex;
mod configuration;

use self::vertex::Vertex;
pub use self::triangle::TriangleNode;
pub use self::configuration::Configuration;

use glium::glutin::VirtualKeyCode;
use glium::index::PrimitiveType;
use glium::{Surface, VertexBuffer, IndexBuffer};
use glium::{Program, Display, DrawParameters};

extern crate rand;
use rand::Rng;

use time::Duration;
use time::PreciseTime;

use cgmath::Vector2;

//Converts a given duration to seconds
fn duration_to_secs(duration: Duration) -> f64 {
    duration.num_nanoseconds().unwrap() as f64 / 1_000_000_000.0
}

pub struct MainState<'a> {

    frame_start: PreciseTime, // Time when the last frame started
    cancelled: bool, // Spacebar pressed?

    //Range is 0..max_progress
    progress: f64,
    max_progress: f64,

    start: PreciseTime, // Time when we started

    configuration: Configuration, // What configuration to use for the current MainState
    next_configuration: Configuration, // What configuration to use for the next MainState

    // Glium stuff
    display: &'a Display,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
}

impl<'a>  MainState<'a>  {

    //Constructor
    pub fn new(display: &Display, configuration: Configuration) -> MainState {

        let mut start = PreciseTime::now();

        //Closure to get time spent in ms and restart the timer
        let mut restart_get = || {
            let now = PreciseTime::now();
            let duration = duration_to_secs(start.to(now));
            start = now;
            duration * 1000.0
        };

        //Generate initial triangles
        let max_triangles = 400_000;
        let mut triangles = configuration.get_triangles();
        let mut time = 0.0;

        //Iteratively subdivide until we reach the limit
        let mut iterations = 0;
        while Self::subdivide(&mut triangles, max_triangles, &mut time) {
            iterations += 1;
        }

        //Measure time spent subdividing
        let time_triangle_subdivision = restart_get();

        //Convert all generated triangles into vertices for the vertex buffer
        let mut vertices = vec!();
        let len = triangles.len() as f32;
        let max_progress: f64 = 10.0; // Amount of time to fade in, in seconds

        for triangle in triangles {
            let time = (triangle.progress / len).powf(1.0 / 3.0) * max_progress as f32;
            vertices.push(Vertex::from_vector2(&triangle.verts[0], time));
            vertices.push(Vertex::from_vector2(&triangle.verts[1], time));
            vertices.push(Vertex::from_vector2(&triangle.verts[2], time));
        }

        //Measure time spent converting
        let time_triangle_conversion = restart_get();

        //Setup vertex and index buffers
        let vertex_buffer = VertexBuffer::new(display, &vertices)
            .expect("Failed to setup vertex buffer");
        let indices: Vec<_> = (0..vertex_buffer.len() as u32).collect();
        let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices[..])
            .expect("Failedd to setup index buffer");

        //Measure time spent setting up buffers
        let time_triangle_buffers = restart_get();

        //Print time spent
        println!("Time spent: \
        \n Triangle subdivision: {:.1} ms ({} subdivisions, {} triangles)\
        \n Triangle conversion: {:.1} ms\
        \n Triangle buffering: {:.1} ms\
        \nTOTAL: {:.1} ms
        ", time_triangle_subdivision, iterations, len, time_triangle_conversion, time_triangle_buffers,
                 time_triangle_subdivision + time_triangle_conversion + time_triangle_buffers);

        MainState {
            frame_start: PreciseTime::now(),
            start: PreciseTime::now(),
            cancelled: false,
            progress: 0.0,
            max_progress,
            configuration,
            next_configuration: configuration, //By default, the next configuration is equal to the current one
            display,
            vertex_buffer,
            index_buffer,
        }
    }

    fn update(&mut self) -> f64 {

        //Calculate delta time (time since last frame)
        let now = PreciseTime::now();
        let delta = duration_to_secs(self.frame_start.to(now));
        self.frame_start = now;

        //Calculate time since creating the triangle
        let time = duration_to_secs(self.start.to(now));

        //After a certain amount of time, trigger a fadeout
        if time > 20.0 { self.transition_to_same(); }

        //Fade in negative direction when space is pressed (fade out)
        if self.cancelled {
            self.progress -= delta * 5.0;
            if self.progress <= 0.0 {
                *self = Self::new(self.display, self.next_configuration); //Move to the next configuration when done fading out
            }
        } else { //Else, fade in
            self.progress += delta;
        }

        //Clamp progress to 0..max_progress
        self.progress = self.progress.min(self.max_progress).max(0.0);

        delta
    }

    pub fn input(&mut self, key: VirtualKeyCode) {

        match key {

            //Space generates a new configuration, identical to the current one
            VirtualKeyCode::Space => { self.transition_to_same(); }

            //1 generates a square, 2 generates a triangle
            VirtualKeyCode::Key1 => { self.transition_to(Configuration::Square); }
            VirtualKeyCode::Key2 => { self.transition_to(Configuration::Triangle); }

            //All other numbers generate an n-gon
            VirtualKeyCode::Key3 => { self.transition_to(Configuration::NGon(3)); }
            VirtualKeyCode::Key4 => { self.transition_to(Configuration::NGon(4)); }
            VirtualKeyCode::Key5 => { self.transition_to(Configuration::NGon(5)); }
            VirtualKeyCode::Key6 => { self.transition_to(Configuration::NGon(6)); }
            VirtualKeyCode::Key7 => { self.transition_to(Configuration::NGon(7)); }
            VirtualKeyCode::Key8 => { self.transition_to(Configuration::NGon(8)); }
            VirtualKeyCode::Key9 => { self.transition_to(Configuration::NGon(9)); }

            _ => {}
        }

    }

    //Trigger fade out and move to the same configuration when done fading out
    fn transition_to_same(&mut self) {
        let conf = self.configuration; //This needs to be done due to the borrow checker
        self.transition_to(conf);
    }

    //Trigger fade out and move to the given configuration when done fading out
    pub fn transition_to(&mut self, next: Configuration) {

        if self.cancelled { return; }

        self.cancelled = true;
        self.next_configuration = next;
    }

    //Subdivide the list of triangles
    //Returns false if triangle limit reached
    fn subdivide(tris: &mut Vec<TriangleNode>, max_triangles: usize, time: &mut f32) -> bool {

        let mut new_tris = Vec::with_capacity(tris.len() / 10);

        //Every triangle has an unique time so we can fade them in sequentially
        let mut next_time = || {
            *time += 1.0;
            *time
        };

        let chance = 0.1;

        for tri_node in tris.iter() {

            let mut rng = rand::thread_rng();

            let p1 = tri_node.verts[0];
            let p2 = tri_node.verts[1];
            let p3 = tri_node.verts[2];

            //Half-way points between corners of the triangle
            let l1: Vector2<f64> = (p1 + p2) / 2.0;
            let l2: Vector2<f64> = (p2 + p3) / 2.0;
            let l3: Vector2<f64> = (p3 + p1) / 2.0;

            //Randomly add subdivided triangles
            if rng.gen::<f32>() < chance { new_tris.push(TriangleNode::new([l1, l2, l3], next_time())); }
            if rng.gen::<f32>() < chance { new_tris.push(TriangleNode::new([p1, l1, l3], next_time())); }
            if rng.gen::<f32>() < chance { new_tris.push(TriangleNode::new([p2, l2, l1], next_time())); }
            if rng.gen::<f32>() < chance { new_tris.push(TriangleNode::new([p3, l3, l2], next_time())); }
        }

        tris.extend(new_tris);

        //Cut off if we have too many triangles and stop the subdivision process
        if tris.len() >= max_triangles {
            tris.truncate(max_triangles);
            return false;
        }

        true
    }

    pub fn draw(&mut self, display: &Display, program: &Program, params: &DrawParameters) {

        //Building the uniforms
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            progress: self.progress as f32
        };

        let delta = self.update();

        //Uncomment to print FPS
        //println!("delta: {} secs ({:.1} fps)", delta, 1.0/delta);

        //Drawing the vertex buffer
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        target.draw(&self.vertex_buffer, &self.index_buffer, program, &uniforms, params)
            .expect("Failed to draw vertex buffer");
        target.finish().expect("Failed to swap buffers");
    }
}

