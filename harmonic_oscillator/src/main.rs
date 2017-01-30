#[macro_use] extern crate conrod;
extern crate find_folder;

use std::time::Duration;
use std::thread;
use std::f32;


//F_net = F_spring + mg
//F_net = - k x_alter - mg + mg
//F_net = - k*x_alter

//F_net = m*(d^2x/dt^2)= -k*x
//Solution:
//x(t) = x_0*cos(wt)+(v_0/w)*sin(wt)
//x(t) = A * cos(w*t+phi)


fn F_spring(k: f32, x_alter: f32, m: f32, g: f32) -> f32{
    return -k * x_alter - m*g;
}

fn F_net(k: f32, x_alter: f32) -> f32{
    return -k*x_alter;
}

fn x(t: f32, A: f32, w: f32, phi: f32)-> f32{
    return A*(w*t+phi).cos();
}

fn main() {
    const WIDTH: u32 = 200;
    const HEIGHT: u32 = 200;
    
    use conrod::{widget, Labelable, Positionable, Sizeable, Widget};
    use conrod::backend::piston::{self, Window, WindowEvents, OpenGL};
    use conrod::backend::piston::event::UpdateEvent;

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    
    
    // Construct the window.
    let mut window: Window = piston::window::WindowSettings::new("Click me!", [WIDTH, HEIGHT])
        .opengl(opengl).exit_on_esc(true).build().unwrap();

    // Create the event loop.
    let mut events = WindowEvents::new();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Generate the widget identifiers.
    widget_ids!(struct Ids { canvas, counter });
    let ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_texture_cache = piston::window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    // The image map describing each of our widget->image mappings (in our case, none).
    //let image_map = conrod::image::Map::new();

    let mut count = 0;

    
    
    // Spring constant
    let k: f32 = 100.0;
    // Hanger mass
    let m: f32 = 20.0;
    // Earth Gravity
    let g: f32 = 9.807; // meters/sec²
    // Strech due to mass
    let dL: f32 = (m*g)/k;
    //
    let w: f32 = (k/m).sqrt();
    
    let phi: f32 = 2.0;
    let A: f32 = 2.0;
    
    let x_alter: f32 = 4.0;
    
    print!("Spring Force: {} \nNet Force: {}", F_spring(k, x_alter, m, g), F_net(k, x_alter) );
    
    let period: f32 = 0.1; //secs
    let timeout: f32 = 5.0; //secs
    let mut t: f32 = 0.0;
    while t<timeout {
        let period_ms = period*1000.0; //msecs
        thread::sleep(Duration::from_millis(period_ms as u64));
        t = t + period;
        
        println!("Position: {}",format!("{:.*}", 3, x(t, A, w, phi)));
    }
}
