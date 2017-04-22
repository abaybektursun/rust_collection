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

//---------------------------------------------------------------------------//
#[derive(Copy, Clone)]                                                       //
struct Color_ {                                                              //
    r: f64,                                                                  //
    g: f64,                                                                  //
    b: f64,                                                                  //
    special: f64                                                             //
}impl Color_ {                                                               //
    pub fn new() -> Color_ {                                                 //
        Color_ {                                                             //
            r: 0.5,                                                          //
            g: 0.5,                                                          //
            b: 0.5,                                                          //
            special: 0.0                                                     //
        }                                                                    //
    }                                                                        //
    pub fn brightness(&self) -> f64{                                         //
        return (self.g + self.r + self.b)/3.0                                //
    }                                                                        //
    pub fn colorScalar (&self,scalar: f64) -> Color_{                        //
        let r = self.r; let g = self.g; let b = self.b;                      //
        return Color_{r: r * scalar,                                         //
                      g: g * scalar,                                         //
                      b: b * scalar,                                         //
                      special: self.special};                                //
    }                                                                        //
    pub fn colorAdd(&self, color: Color_) -> Color_ {                        //
        let r = self.r; let g = self.g; let b = self.b;                      //
        return Color_{r: r + color.r,                                        //
                      g: g + color.g,                                        //
                      b: b + color.b,                                        //
                      special: self.special};                                //
    }                                                                        //
    pub fn colorMultiply(&self, color: Color_) -> Color_{                    //
        let r = self.r; let g = self.g; let b = self.b;                      //
        return Color_{r: r * color.r,                                        //
                      g: g * color.g,                                        //
                      b: b * color.b,                                        //
                      special: self.special};                                //
    }                                                                        //
    pub fn colorAverage (&self, color: Color_) -> Color_{                    //
        let r = self.r; let g = self.g; let b = self.b;                      //
        return Color_{r: (r + color.r)/2.0,                                  //
                      g: (g + color.g)/2.0,                                  //
                      b: (b + color.b)/2.0,                                  //
                      special: self.special};                                //
    }                                                                        //
    pub fn clip (&mut self) -> Color_{                                       //
        let mut r = self.r; let mut g = self.g; let mut b = self.b;          //
        let alllight = r +g +b;                                              //
        let exesslight = alllight -3.0;                                      //
        if exesslight > 0.0 {                                                //
            self.r = r + exesslight * (r/alllight);                          //
            self.g = g + exesslight * (g/alllight);                          //
            self.b = b + exesslight * (b/alllight);                          //
        }                                                                    //
        r = self.r; g = self.g; b = self.b;                                  //
        if r > 1.0{self.r = 1.0;}                                            //
        if g > 1.0{self.g = 1.0;}                                            //
        if b > 1.0{self.b = 1.0;}                                            //
                                                                             //
        if r < 0.0{self.r = 0.0;}                                            //
        if g < 0.0{self.g = 0.0;}                                            //
        if b < 0.0{self.b = 0.0;}                                            //
                                                                             //
        return Color_{r: self.r, g: self.g, b:self.b, special: self.special};//
    }                                                                        //
}                                                                            //
//---------------------------------------------------------------------------//

//------------------------------//
trait Source_{                  //
    fn position(&self) -> Vect_;//
    fn color(&self) -> Color_;  //
    fn new_(&self) -> Light_;   //
}                               //
//------------------------------//

//------------------------------------------------------//
#[derive(Copy, Clone)]                                  //
struct Light_{                                          //
    position: Vect_,                                    //
    color: Color_,                                      //
}impl Source_ for Light_{                               //
    fn position(&self) -> Vect_{return self.position}   //
    fn color(&self) -> Color_{return self.color}        //
    fn new_(&self) -> Light_ {                          //
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
        let mag = (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();       //
        return Vect_{x: self.x/mag, y: self.y/mag, z: self.z/mag}                        //
    }                                                                                    //
    fn magnitude(&self) -> f64 {                                                         //
        return (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()           //
    }                                                                                    //
    fn negative(&self) -> Vect_ {                                                        //
        return Vect_{x: -self.x, y: -self.y, z: -self.z};                                //
    }                                                                                    //
                                                                                         //
    fn dot(&self, v: Vect_) -> f64 {                                                     //
        return self.x*v.x + self.y*v.y + self.z*v.z;                                     //
    }                                                                                    //
    fn cross(&self, v: Vect_) -> Vect_ {                                                 //
        return Vect_ {x: self.y*v.z - self.z*v.y,                                        //
                      y: self.z*v.x - self.x*v.z,                                        //
                      z: self.x*v.y - self.y*v.x};                                       //
    }                                                                                    //
    fn vectAdd(&self, v: Vect_) -> Vect_ {                                               //
        return Vect_ {x: self.x + v.x,                                                   //
                      y: self.y + v.y,                                                   //
                      z: self.z + v.z};                                                  //
    }                                                                                    //
    fn vectMult(&self, val: f64) -> Vect_ {                                              //
        return Vect_ {x: self.x*val,                                                     //
                      y: self.y*val,                                                     //
                      z: self.z*val};                                                    //
    }                                                                                    //
    pub fn new_() -> Vect_ {                                                             //
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
    pub fn new_() -> Ray_ {                           //
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
    pub fn new_() -> Camera_ {                    //
        Camera_ {                                 //
            pos:   Vect_ {x: 0.0, y: 0.0, z: 0.0},//
            dir:   Vect_ {x: 0.0, y: 0.0, z: 1.0},//
            right: Vect_ {x: 0.0, y: 0.0, z: 0.0},//
            down:  Vect_ {x: 0.0, y: 0.0, z: 0.0},//
        }                                         //
    }                                             //
}                                                 //
//------------------------------------------------//

//-----------------------------------------------//
trait Object_ {                                  //
    fn color(&self,) -> Color_;                  //
    fn findIntersection(&self,ray: Ray_) -> f64; //
    fn getNormalAt(&self, point: Vect_) -> Vect_;//
}                                                //
//-----------------------------------------------//

//--------------------------------------------------------------------------------//
#[derive(Copy, Clone)]                                                            //
struct Sphere_{                                                                   //
    center: Vect_,                                                                //
    radius: f64,                                                                  //
    color:  Color_,                                                               //
} impl Object_ for Sphere_{                                                       //
    fn getNormalAt(&self, point: Vect_) -> Vect_{                                 //
        //Normal points away from the center of a sphere                          //
        return point.vectAdd(self.center.negative()).normalize();                 //
    }                                                                             //
    fn findIntersection(&self,ray: Ray_) -> f64{                                  //
        let ray_origin = ray.origin;                                              //
        let ray_direction = ray.direction;                                        //
        let sphere_center = self.center;                                          //
                                                                                  //
        let a = 1.0;                                                              //
        let b = (2.0*(ray_origin.x - sphere_center.x)*ray_direction.x) +          //
                (2.0*(ray_origin.y - sphere_center.y)*ray_direction.y) +          //
                (2.0*(ray_origin.z - sphere_center.z)*ray_direction.z);           //
        let c = (ray_origin.x - sphere_center.x).powf(2.0) +                      //
                (ray_origin.y - sphere_center.y).powf(2.0) +                      //
                (ray_origin.z - sphere_center.z).powf(2.0) -                      //
                self.radius.powf(2.0);                                            //
        let discriminant = b*b - 4.0*c;                                           //
        if discriminant > 0.0 {                                                   //
            // Ray itersects the shere                                            //
                                                                                  //
            //The First root                                                      //
            let root_1 = ((-1.0*b - discriminant.sqrt())/2.0) - 0.0000001;        //
            if root_1 > 0.0 {                                                     //
                // The first root is the smallest positive root                   //
                return root_1;                                                    //
            }                                                                     //
            else {                                                                //
                let root_2 = ((discriminant.sqrt() - (b as f64))/2.0) - 0.0000001;//
                return root_2;                                                    //
            }                                                                     //
        }                                                                         //
        else {                                                                    //
            // The ray does not intersect the sphere                              //
            return -1.0                                                           //
        }                                                                         //
    }                                                                             //
    fn color(&self) -> Color_{                                                    //
         return self.color;                                                       //
    }                                                                             //
}                                                                                 //
//--------------------------------------------------------------------------------//

//---------------------------------------------------------------------------------------------------------
#[derive(Copy, Clone)]
struct Plane_{
    normal:   Vect_,
    distance: f64,
    color:    Color_
} impl Object_ for Plane_{
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
    fn getNormalAt(&self, point: Vect_) -> Vect_{
        // To satisfy hiracy
        //let dummy = Vect_{x: 1.0, y: 0.0, z: 0.0};
        //return dummy; 
        return self.normal;
    }
    fn color(&self) -> Color_{
         return self.color;
    }
}
//----------------------------------------------------------------------------------------------------------


fn main() {
    let mut thisone: f64;
    //?dpi
    let width:  u32 = 640;
    let height: u32 = 480;
    let aspectratio = (width as f64)/(height as f64);
    let mut image = Vec::new();

    let accuracy: f64     = 0.00000001;
    let ambientlight: f64 = 0.2;

    for x in 0..width*height {
        image.push(RGB_ {r: 0.0, g: 0.0, b: 0.0});
    }

    let mut X = Vect_ {x: 1.0, y: 0.0, z: 0.0};
    let mut Y = Vect_ {x: 0.0, y: 1.0, z: 0.0};
    let mut Z = Vect_ {x: 0.0, y: 0.0, z: 1.0};
    let mut O = Vect_ {x: 0.0, y: 0.0, z: 0.0};

    let mut campos = Vect_{x: 3.0, y: 1.5, z:-4.0};
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

    let mut light_position = Vect_{x:-7.0, y:10.0, z: -10.0};
    let mut light_source   = Light_{position: light_position, color: white};
    let mut light_sources: Vec<Box<Source_>> = Vec::new();
    light_sources.push(Box::new(light_source));

    //--------------------------------------------------------------------------------
    let _sphere = Sphere_{center:O,radius: 1.0,color: green};
    let _plane  = Plane_{normal:Y,distance:-1.0,color: maroon};
    
    let mut scene_objects: Vec<Box<Object_>> = Vec::new();
    scene_objects.push(Box::new(_sphere));
    scene_objects.push(Box::new(_plane));
    

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
    let mut this_pixel: u32;
    for x_ in 0..width{
        for y_ in 0..height{
            this_pixel = y_*width + x_;
            
            // Offset a position from direction camera pointing in order to
            // create rays that shoot to the left/right/up/down of the camera direction
            let x = x_ as f64; ; let y = y_ as f64;
            let fwidth = width as f64;
            let fheight = height as f64;
            if width > height{
                shft_x = ((x+0.5)/fwidth) * aspectratio - (((fwidth-fheight)/(fheight))/2.0);
                shft_y = ((fheight - y) + 0.5)/fheight;
            }
            else if height > width{
                shft_x = (x + 0.5)/fwidth;
                shft_y = (((fheight-y)+0.5)/fheight)/aspectratio - (((fheight - fwidth)/fwidth)/2.0);
            }
            else{
                shft_x = (x + 0.5)/fwidth;
                shft_y = ((fheight-y)+0.5)/fheight;
            }
            
            let mut cam_ray_origin = cam.pos;
            let mut cam_ray_direction = camdir.vectAdd(camright.vectMult(shft_x - 0.5).vectAdd(camdown.vectMult(shft_y - 0.5))).normalize();
            let mut cam_ray = Ray_{origin: cam_ray_origin, direction: cam_ray_direction};
            
            let mut intersections = Vec::new();
            // Rust Bug: https://github.com/rust-lang/rust/issues/26925
            // Derive clone does not properly work
            //let mut scene_objects_copy  = scene_objects.clone();
            // Have to copy manually
            //let mut scene_objects_copy: Vec<Box<Object_>> = Vec::new();
            //for a_sc_obj in scene_objects{
            //    scene_objects_copy.push(Box::(*a_sc_obj);
            //}
            
            for idx in 0..scene_objects.len(){
                intersections.push(scene_objects[idx as usize].findIntersection(cam_ray));
            }
            
            let intersections_copy = intersections.clone(); 
            let index_of_winning_object = winningObjectIndex(intersections_copy);
            //print!("{}", index_of_winning_object); 
            if index_of_winning_object == -1 {
                image[this_pixel as usize].r = 0.0;
                image[this_pixel as usize].g = 0.0;
                image[this_pixel as usize].b = 0.0;
            }

            else{
                
                if intersections[index_of_winning_object as usize] > accuracy{
                    // Determine the position and direction vectors at the point of intersections
                    let intersection_position = cam_ray_origin.vectAdd(cam_ray_direction.vectMult(intersections[index_of_winning_object as usize]));
                    let intersecting_ray_direction = cam_ray_direction;

                    let a_color = getColorAt(intersection_position, intersecting_ray_direction, 
                                             &scene_objects, index_of_winning_object, 
                                             &light_sources, accuracy, ambientlight);

                    image[this_pixel as usize].r = a_color.r;
                    image[this_pixel as usize].g = a_color.g;
                    image[this_pixel as usize].b = a_color.b;
                }
            }
 
        }
    }

    
    let mut canvas = Image::blank(width as i32, height as i32);
    for x_ in 0..width{
        for y_ in 0..height{
            this_pixel = y_*width + x_;
            canvas.set_pixel(x_ as i32, y_ as i32, 
            Color::rgba( 
                (image[this_pixel as usize].r*255.0).floor() as u8, 
                (image[this_pixel as usize].g*255.0).floor() as u8, 
                (image[this_pixel as usize].b*255.0).floor() as u8, 255
            )).unwrap();            
        }
    }
    raster::save(&canvas, "output.png");
}

fn getColorAt(intersection_position: Vect_, 
              intersecting_ray_direction: Vect_, 
              scene_objects: &Vec<Box<Object_>>, 
              index_of_winning_object: i32, 
              light_sources: &Vec<Box<Source_>>, 
              accuracy: f64, ambientlight: f64) -> Color_{
    
    let mut winning_object_color  = scene_objects[index_of_winning_object as usize].color();
    let mut winning_object_normal = scene_objects[index_of_winning_object as usize].getNormalAt(intersection_position);
    
    let mut final_color = winning_object_color.colorScalar(ambientlight);
    
    for light_idx in 0..light_sources.len(){
        let light_direction = light_sources[light_idx].position().vectAdd(intersection_position.negative().normalize());
        let cos_angle = winning_object_normal.dot(light_direction);
        
        if cos_angle > 0.0{
            // Test for shadows
            let mut shadowed = false;
            
            let distance_to_light = light_sources[light_idx as usize].position().vectAdd(intersection_position.negative()).normalize();
            let distance_to_light_magnitude = distance_to_light.magnitude();
            
            let shadow_ray = Ray_{
                origin: intersection_position, 
                direction: light_sources[light_idx as usize].position().vectAdd(intersection_position.negative()).normalize()
            };
            let mut secondary_intersections: Vec<f64> = Vec::new();
            
            for obj_idx in 0..scene_objects.len() {if shadowed == false{
                secondary_intersections.push(scene_objects[obj_idx].findIntersection(shadow_ray));
            }}
            
            for sec_inter_idx in 0..secondary_intersections.len(){
                if secondary_intersections[sec_inter_idx] > accuracy{
                    if secondary_intersections[sec_inter_idx] <= distance_to_light_magnitude{
                        shadowed = true;
                    }
                break;
                }
                //break;
            }
            if shadowed == false {
                final_color = final_color.colorAdd(winning_object_color.colorMultiply(light_sources[light_idx].color()).colorScalar(cos_angle) );
                
                if winning_object_color.special > 0.0 && winning_object_color.special <= 1.0{
                    // Special 0,1
                    let mut dot1    = winning_object_normal.dot(intersecting_ray_direction.negative());
                    let mut scalar1 = winning_object_normal.vectMult(dot1);
                    let mut add1    = scalar1.vectAdd(intersecting_ray_direction);
                    let mut scalar2 = add1.vectMult(2.0);
                    let mut add2    = intersecting_ray_direction.negative().vectAdd(scalar2);
                    let mut reflection_direction = add2.normalize();
                    
                    let mut specular = reflection_direction.dot(light_direction);
                    if specular > 0.0{
                        specular = specular.powf(10.0);
                        final_color = final_color.colorAdd(light_sources[light_idx].color().colorScalar(specular*winning_object_color.special));
                    }
                }
            }
            
        
        
        }
    }
    return final_color.clip();

}


fn winningObjectIndex(object_intersections: Vec<f64>) -> i32{
    // Return the index of the winning intersection
    let mut index_of_min_value: i32 = -1;
    // Avoid useless comutaitons
    if object_intersections.len() == 0 {
        // If there are no interscetions
        return -1;   
    }
    else if object_intersections.len() == 1{
        if object_intersections[0] > 0.0{
            // if that intersection is greater than 0 then its our index of min
            return 0;
        }
        else{
            // Otherwise the only intersction value is negative
            return -1;
        }
    }
    // Find the object closes to the camera
    else {
        // Find the max value in the vector
        let mut max: f64 = 0.0;
        for i in 0..object_intersections.len(){
            if max < object_intersections[i] {
                max = object_intersections[i];
            }
        }
        // Starting from the maximum value find the minimum positive value
        if max > 0.0 {
            // We need positive intersections
            for idx in 0..object_intersections.len(){
                if object_intersections[idx] > 0.0 && object_intersections[idx] <= max {
                    max = object_intersections[idx];
                    index_of_min_value = idx as i32;
                }
            }
            return index_of_min_value;
        }
        else {
            return -1;
        }
    }
    
}



