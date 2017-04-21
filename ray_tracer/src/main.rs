#[macro_use]
extern crate raster; 
use raster::Image;
use raster::Color;

//--------------------//
#[derive(Copy, Clone)]//
struct RGB_ {         //
    r: f64,           //
    g: f64,           //
    b: f64,           //
}                     //
//--------------------//

//--------------------------//
#[derive(Copy, Clone)]      //
struct Color_ {             //
    r: f64,                 //
    g: f64,                 //
    b: f64,                 //
    special: f64            //
}impl Color_ {              //
    pub fn new() -> Color_ {//
        Color_ {            //
            r: 0.5,         //
            g: 0.5,         //
            b: 0.5,         //
            special: 0.0    //
        }                   //
    }                       //
}                           //
//--------------------------//

//------------------------------------------------------//
#[derive(Copy, Clone)]                                  //
struct Light_{                                          //
    position: Vect_,                                    //
    color: Color_,                                      //
}impl Light_{                                           //
    pub fn new() -> Light_ {                            //
        Light_ {                                        //
            position: Vect_{x:0.0, y:0.0, z:0.0},       //
            color: Color_{r:1.0,g:1.0,b:1.0,special:0.0}//
        }                                               //
    }                                                   //
}                                                       //
//------------------------------------------------------//

//---------------------------------------------------------------------------------------//
#[derive(Copy, Clone)]                                                                   //
struct Vect_ {                                                                           //
    x: f64,                                                                              //
    y: f64,                                                                              //
    z: f64,                                                                              //
}impl Vect_ {                                                                            //
    fn normalize(&self) -> Vect_ {                                                       //
        let mag = (&self.x.powf(2.0) + &self.y.powf(2.0) + &self.z.powf(2.0)).powf(0.5); //
        return Vect_{x: &self.x/mag, y: &self.y/mag, z: &self.z/mag}                     //
    }                                                                                    //
    fn magnitude(&self) -> f64 {                                                         //
        return (&self.x.powf(2.0) + &self.y.powf(2.0) + &self.z.powf(2.0)).powf(0.5)     //
    }                                                                                    //
    fn negative(&self) -> Vect_ {                                                        //
        return Vect_{x: -&self.x, y: -&self.y, z: -&self.z};                             //
    }                                                                                    //
                                                                                         //
    fn dot(&self, v: Vect_) -> f64 {                                                     //
        return &self.x*v.x + &self.y*v.y + &self.z*v.z;                                  //
    }                                                                                    //
    fn cross(&self, v: Vect_) -> Vect_ {                                                 //
        return Vect_ {x: &self.y*v.z - &self.z*v.y,                                      //
                      y: &self.z*v.x - &self.x*v.z,                                      //
                      z: &self.x*v.y - &self.y*v.x};                                     //
    }                                                                                    //
    fn vectAdd(&self, v: Vect_) -> Vect_ {                                               //
        return Vect_ {x: &self.x + v.x,                                                  //
                      y: &self.y + v.y,                                                  //
                      z: &self.z + v.z};                                                 //
    }                                                                                    //
    fn vectMult(&self, val: f64) -> Vect_ {                                              //
        return Vect_ {x: &self.x*val,                                                    //
                      y: &self.y*val,                                                    //
                      z: &self.z*val};                                                   //
    }                                                                                    //
    pub fn new() -> Vect_ {                                                              //
        Vect_ {                                                                          //
            x: 0.0,                                                                      //
            y: 0.0,                                                                      //
            z: 0.0,                                                                      //
        }                                                                                //
    }                                                                                    //
}                                                                                        //
//---------------------------------------------------------------------------------------//

//----------------------------------------------------//
#[derive(Copy, Clone)]                                //
struct Ray_ {                                         //
    origin: Vect_,                                    //
    direction: Vect_,                                 //
}impl Ray_ {                                          //
    //fn value(&self) -> &f64 { &self.x }             //
    pub fn new() -> Ray_ {                            //
        Ray_ {                                        //
            origin:    Vect_ {x: 0.0, y: 0.0, z: 0.0},//
            direction: Vect_ {x: 1.0, y: 0.0, z: 0.0},//
        }                                             //
    }                                                 //
}                                                     //
//----------------------------------------------------//

//------------------------------------------------//
#[derive(Copy, Clone)]                            //
struct Camera_ {                                  //
    pos:   Vect_,                                 //
    dir:   Vect_,                                 //
    right: Vect_,                                 //
    down:  Vect_,                                 //
}                                                 //
impl Camera_ {                                    //
    //fn value(&self) -> &f64 { &self.x }         //
    pub fn new() -> Camera_ {                     //
        Camera_ {                                 //
            pos:   Vect_ {x: 0.0, y: 0.0, z: 0.0},//
            dir:   Vect_ {x: 0.0, y: 0.0, z: 1.0},//
            right: Vect_ {x: 0.0, y: 0.0, z: 0.0},//
            down:  Vect_ {x: 0.0, y: 0.0, z: 0.0},//
        }                                         //
    }                                             //
}                                                 //
//------------------------------------------------//

//----------------------------------------//
trait Object_ {                           //
    fn getColor() -> Color_;              //
    fn findIntersection(ray: Ray_) -> f64;//
}                                         //
//----------------------------------------//

//------------------------------------------------------//
#[derive(Copy, Clone)]                                  //
struct Sphere_{                                         //
    center: Vect_,                                      //
    radius: f64,                                        //
    color:  Color_,                                     //
} impl Sphere_{                                         //
    pub fn new() -> Sphere_ {                           //
        Sphere_ {                                       //
            center: Vect_{x:0.0,y:0.0,z:0.0},           //
            radius: 1.0,                                //
            color: Color_{r:0.5,g:0.5,b:1.0,special:0.0}//
        }                                               //
    }                                                   //
}                                                       //
//------------------------------------------------------//


//impl Object for Sphere_{
//    fn getColor() -> Color_{
//        return Color_{r:0.0,g:0.0,b:0.0,special:0.0}
//    }
//    fn findIntersection(ray: Ray_) -> f64{
//        return Ray_{origin:Vect_{},
//                    direction:Vect_{}}
//    }
//}

struct Plane_{
    normal:   Vect_,
    distance: f64,
    color:    Color_
} impl Plane_{
    pub fn new() -> Plane_ {
        Plane_ {
            normal: Vect_{x:1.0,y:0.0,z:0.0},
            distance: 1.0,
            color: Color_{r:0.5,g:0.5,b:0.5,special:0.0}
        }
    }
    fn getNormalAt(&self, point: Vect_) -> Vect_{
        return self.normal;
    }
    fn findIntersection(&self,ray: Ray_) -> f64{
        let r_direct = ray.direction;
        // Dot product of the direction of the ray and the normal of the plane
        let a = r_direct.dot(self.normal);
        // Ray and plane are orthogonal
        if a == 0.0{
            return -1.0
        }else{
            // Dot product of normal and the vector opposite direction of the origin of the ray
            let b = &self.normal.dot(ray.origin.vectAdd(self.normal.vectMult(self.distance).negative()));
            return -1.0*b/a
        }
    }
}

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as i32, $h as i32)
    )
);

fn main() {
    let mut thisone: f64;
    //?dpi
    let width:  u32 = 640;
    let height: u32 = 480;
    let aspectratio = (width as f64)/(height as f64);
    let mut image = Vec::new();

    for x in 0..width*height {
        image.push(Vect_ {x: 0.0, y: 0.0, z: 0.0});
    }

    let mut X = Vect_ {x: 1.0, y: 0.0, z: 0.0};
    let mut Y = Vect_ {x: 0.0, y: 1.0, z: 0.0};
    let mut Z = Vect_ {x: 0.0, y: 0.0, z: 1.0};
    let mut O = Vect_ {x: 0.0, y: 0.0, z: 0.0};

    let mut campos = Vect_{x: 3.0, y: 1.0, z:-5.2};
    let mut look_point = Vect_{x: 0.0, y: 0.0, z: 0.0};
    // Difference between camera position and the point we are aiming for
    let mut diff_between = Vect_ {x: campos.x - look_point.x, y: campos.y - look_point.y, z: campos.z - look_point.z};
    // Normalized vector opposite direction of diff_between
    let mut camdir   = diff_between.negative().normalize();
    let mut camright = Y.cross(camdir).normalize();
    let mut camdown  = camright.cross(camdir);
    let mut cam      = Camera_{pos:campos, dir:camdir, right:camright, down:camdown};

    let white  = Color_{r:1.0,g:1.0, b:1.0, special:0.0};
    let green  = Color_{r:0.5,g:1.0, b:0.5, special:0.3};
    let gray   = Color_{r:0.5,g:0.5, b:0.5, special:0.0};
    let maroon = Color_{r:0.5,g:0.25,b:0.25,special:0.0};
    let black  = Color_{r:0.0,g:0.0, b:0.0, special:0.0};

    let light_position = Vect_{x:-7.0, y:10.0, z: -10.0};
    let light_source   = Light_{position: light_position, color: white};

    //--------------------------------------------------------------------------------
    let _sphere = Sphere_{center:O,radius: 1.0,color: green};
    let _plane  = Plane_{normal:Y,distance:-1.0,color: maroon};
    let mut shft_x: f64; let mut shft_y: f64;

    // Offset a position from direction camera pointing in order to
    // create rays that shoot to the left/right/up/down of the camera direction
    //if width > height{
    //    shft_x = ((x+0.5)/width) * aspectratio - (((width-height)/(height as f64))/2);
    //    shft_y = ((height - y) + 0.5)/height;
    //}
    //else if height > width{
    //    shft_x = ((x + 0.5)/width);
    //    shft_y = (((height-y)+0.5)/height)/aspectratio - (((height - width)/(width as f64))/2);
    //}
    //else{
    //    shft_x = (x + 0.5)/width;
    //    shft_y = ((height-y)+0.5)/height;
    //}
    //--------------------------------------------------------------------------------

    //use glium::{DisplayBuild, Surface};
    //let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    //loop {
    //    let mut target = display.draw();
    //    //target.clear_color(0.0, 1.0, 1.0, 1.0);

    //    target.finish().unwrap();

    //    //let pixels: Vec<Vec<(u8, u8, u8, u8)>> = display.read_front_buffer();

    //    for ev in display.poll_events() {
    //        match ev {
    //            glium::glutin::Event::Closed => return,
    //            _ => ()
    //        }
    //    }
    //}

    let mut canvas = Image::blank(800, 600);
    for x_ in 0..800{
        for y_ in 0..600{
            canvas.set_pixel(x_, y_, Color::rgba((x_%255) as u8, 0, 0, (x_%255) as u8)).unwrap();
        }
    }
    raster::save(&canvas, "test_tmp.png");

    //let mut canvas = Image::blank(200, 100); // Mutable 200x100 image with black background

    //canvas.set_pixel(10, 10, Color::rgba(255, 0, 0, 255)).unwrap(); // Set pixel at 10, 10 to red

    //raster::save(&canvas, "test_tmp.png"); // Save


}

