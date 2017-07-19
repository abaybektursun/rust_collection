extern crate image;
extern crate num_cpus;

use std::thread;
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
    let CORES = num_cpus::get();
    println!("Number of cores: {}", CORES);
    
    // Square image
    const RESOLUTION: usize = 3000;
    let PLANE: f64 = 1.0;
    let STEP       = 2.0 * PLANE/(RESOLUTION as f64);

    let mut img = image::ImageBuffer::new(RESOLUTION as u32, RESOLUTION as u32);
    
    let mut workers = vec![];

    //let mut image_2D: [[u8; RESOLUTION]; RESOLUTION] = [[120; RESOLUTION]; RESOLUTION];
    let mut image_2D_raw = vec![163; RESOLUTION * RESOLUTION];
    let mut image_2D_base: Vec<_> = image_2D_raw.as_mut_slice().chunks_mut(RESOLUTION).collect();
    let mut image_2D: &mut [&mut [_]] = image_2D_base.as_mut_slice();


    let work = (RESOLUTION)/CORES;
    for i in 0..CORES {
        workers.push(thread::spawn( move || {
            image_2D_raw; image_2D_base;
            let x_start = RESOLUTION - work * i - 1;
            let y_start = RESOLUTION - work * i - 1;
            let x_end   = RESOLUTION - work * (i + 1);
            let y_end   = RESOLUTION - work * (i + 1);
            println!("x_start:{} \t x_end:{}\n", x_start, x_end);
            for x_ in x_start..x_end{
                for y_ in y_start..y_end {
                    let r = PLANE - STEP * (x_ as f64);
                    let l = PLANE - STEP * (y_ as f64);
                    image_2D[x_][y_] = mandelbrot( Complex{real: r, lateral: l} );
                }
            }
        }
        ));
        workers.push(thread::spawn( || {
            println!("My NaMe IS jeFF");
        }));
    }

    for a_worker in workers { a_worker.join(); }

    for (x_, y_, pix) in img.enumerate_pixels_mut() {
        *pix = image::Luma([image_2D[x_ as usize][y_ as usize]]);
    }
    let ref mut img_file = File::create(&Path::new("mandel.png")).unwrap();
    image::ImageLuma8(img).save(img_file, image::PNG);
}

fn mandelbrot(num: Complex) -> u8 {
    let mut z = num;
    let mut iters: u8 = 0;
    for n in 1..255 {
        if z.abs() > 2.0 { break; }
        z = (z.mul(&z) as Complex).add(&num);
        iters = n;
    }
    return iters;
}
