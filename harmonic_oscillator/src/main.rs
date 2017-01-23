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
        
        println!("Position: {}",x(t, A, w, phi));
    }
}
