mod main_state;
use main_state::MainState;
use main_state::Configuration;

#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate rand;
extern crate time;

use glium::glutin::*;
use glium::{Blend, DrawParameters, Display};

use std::io::prelude::*;
use std::fs::File;

fn load(path: &str) -> String {
    let mut file = File::open(path).expect(&format!("Unable to open the file {}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(&format!("Unable to read the file {}", path));
    contents
}


fn main() {

    //Setup glium stuff
    let mut events_loop = EventsLoop::new();

    let window = WindowBuilder::new()
        .with_fullscreen(Some(events_loop.get_primary_monitor()))
        .with_title("Recursive Geometry");

    let context = ContextBuilder::new()
        .with_multisampling(4) //AA works!
        .with_vsync(true);

    let display = Display::new(window, context, &events_loop)
        .expect("Failed to setup glium");

    //Hide cursor
    if let Err(err) = (*display.gl_window()).set_cursor_state(CursorState::Hide) {
        println!("Failed to hide the cursor: {}", err);
    }

    //Compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: &load("vertex140.vert"),
            fragment: &load("fragment140.frag"),
        }
    ).expect("Failed to setup shaders");

    let mut params = DrawParameters {
        blend: Blend::alpha_blending(),
        ..Default::default()
    };

    //Setup main state
    let mut main_state = MainState::new(&display, Configuration::Triangle);
    let mut closed = false;

    while !closed {

        main_state.draw(&display, &program, &params);

        events_loop.poll_events(|ev| {

            if let Event::WindowEvent { event, .. } = ev {

                match event {

                    WindowEvent::KeyboardInput { input, .. } => {
                        match input{

                            //Close when escape is pressed
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: ElementState::Released, ..
                            } => { closed = true; }

                            //Delegate all other released keys to the main state
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state: ElementState::Released, ..
                            }  => { main_state.input(keycode); }

                            _ => {}

                        }
                    }

                    WindowEvent::Resized(w, h) => {

                        //Square viewport to avoid squashing and center the viewport in the window
                        let s = w.min(h);
                        params.viewport = Some(glium::Rect {
                            left: (f64::from(w - s) / 2.0).round() as u32,
                            bottom: (f64::from(h - s) / 2.0).round() as u32,
                            width: s,
                            height: s,
                        });
                    }

                    //Close when closed
                    WindowEvent::Closed => closed = true,

                    _ => (),
                }
            }
        });
    }
}