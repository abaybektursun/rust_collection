extern crate image;

use std::fs::File;
use std::path::Path;
use image::GenericImage;

#[derive(Copy, Clone)]
struct Complex{
    real:    f64,
    lateral: f64 // a.k.a imaginary
}
impl Complex{
    fn add(&self, num: &Complex) -> Complex{
       Complex {
           real:    self.real    + num.real,
           lateral: self.lateral + num.lateral 
       }
    }
    fn sub(&self, num: &Complex) -> Complex{
       Complex {
           real:    self.real    - num.real,
           lateral: self.lateral - num.lateral
       }
    }
    fn mul(&self, num: &Complex) -> Complex{
       Complex {
           real:    self.real * num.real + (-1.0 * self.lateral * num.lateral),
           lateral: self.real * num.lateral + self.lateral * num.real  
       }
    }
    fn abs(&self) -> f64 {
        ( (self.real.powi(2) + self.real.powi(2) ) as f64).sqrt()
    }
}

fn main() 
{
    let zero = Complex {real: 0.0, lateral: -1.0};
    mandelbrot(zero);
    
    // Square image
    let RESOLUTION = 900;
    let PLANE: f64 = 2.0;
    let STEP       = 2.0 * PLANE/(RESOLUTION as f64);

    let mut img = image::ImageBuffer::new(RESOLUTION, RESOLUTION);
    for (x, y, pix) in img.enumerate_pixels_mut() {
        let r = PLANE - STEP * (x as f64);
        let i = PLANE - STEP * (y as f64);
        let iters = mandelbrot(Complex{real: r, lateral: i});
        
        *pix = image::Luma([iters]);
    }
    let ref mut img_file = File::create(&Path::new("mandel.png")).unwrap();
    image::ImageLuma8(img).save(img_file, image::PNG);
}

fn mandelbrot(num: Complex) -> u8 {
    let mut z = num;
    //let mut mult = z;
    let mut iters: u8 = 0;
    for n in 1..255 {
        if z.abs() > 2.0 { break; }
        //mult = z.mul(&z);
        z = (z.mul(&z) as Complex).add(&num);
        iters = n;
    }
    return iters;
}
