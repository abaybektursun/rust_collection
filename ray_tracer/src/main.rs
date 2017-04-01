#[macro_use]
extern crate glium;

use glium::Surface;

struct RGB {
    r: f32,
    g: f32,
    b: f32,
}

struct Vect_ {
    x: u32,
    y: u32,
    z: u32,
}
impl Vect_ {
    //fn value(&self) -> &f64 { &self.x }
    pub fn new() -> Vect_ {
        Vect_ {
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

struct Ray_ {
    origin: Vect_,
    direction: Vect_,
}
impl Ray_ {
    //fn value(&self) -> &f64 { &self.x }
    pub fn new() -> Ray_ {
        Ray_ {
            origin:    Vect_ {x: 0, y: 0, z: 0,},
            direction: Vect_ {x: 1, y: 0, z: 0,},
        }
    }
}


fn main() {
    use glium::DisplayBuild;

    //let display = glium::glutin::WindowBuilder::new()
    //    .with_dimensions(1024, 768)
    //    .with_title(format!("Hello world"))
    //    .build_glium()
    //    .unwrap();
    
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    
    
    
    
    loop {
        let mut col_: f32 = 0.0;
        let mut target = display.draw();
        target.clear_color(0.0, col_, 1.0-col_, 1.0);
        target.finish().unwrap();
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,   // the window has been closed by the user
                _ => ()
            }
        }
    }
}