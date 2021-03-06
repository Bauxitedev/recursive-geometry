## Recursive Geometry

This program generates geometry and recursively subdivides it into triangles. It doesn't really do anything useful, it just looks pretty.
Run it on a big screen, use it as screen saver, do whatever you want. 

It's written in Rust and uses the [glium](https://github.com/glium/glium) library for drawing to the screen.

Right now the subdivision process is a little slow, I tried parallelizing it to no avail. See [this function](https://github.com/Bauxitedev/recursive-geometry/blob/master/src/main_state/mod.rs#L195), let me know if you have ideas on how to speed it up.

### Demonstration

Here's what it looks like starting with a triangle and a hexagon:

[![](examples/triangle.gif)](examples/triangle.gif) [![](examples/hexagon.gif)](examples/hexagon.gif)

[Here's a video of it in action.](https://www.youtube.com/watch?v=bhWzMR56joc)

### Controls

Key          | Action
------------ | -------------
Esc          | Quit
Space        | Generate new geometry based on the current one
1            | Generate a single triangle
2            | Generate a square consisting of two triangles
3-9          | Generate a regular polygon with the given amount of edges, in the form of a triangle fan

### Running

[Download it here](https://github.com/Bauxitedev/recursive-geometry/releases). Windows & Linux only for now, but you can build it yourself to run it on macOS, see below.

It requires a modern GPU supporting at least OpenGL 3.1.

### Compilation

To build it yourself,  [install rustup](https://www.rustup.rs/), then clone the repository and run `cargo run --release` in the `recursive-geometry` folder.
