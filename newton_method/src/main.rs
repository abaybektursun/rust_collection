use std::f64;

fn f(x: f64)->f64{
    return 3.0*x.powf(5.0)-6.0*x.powf(4.0)+4.0*x.powf(3.0)-4.0*x.powf(2.0)-3.0*x-3.0;
}
    

fn dfdx(x: f64)->f64{
    return 15.0*x.powf(4.0)-24.0*x.powf(3.0)+12.0*x.powf(2.0)-8.0*x-3.0;
}

fn main() {
    let iter = 3;
    println!(
    "f({}) = {}",0,f(0.0)
    );

    println!(
    "f({}) = {}",2,f(2.0)
    );

    let mut x = Vec::new();
    x.push(0.0);

    for x_idx in 1..iter {
        let val = x[x_idx-1] - f(x[x_idx-1])/dfdx(x[x_idx-1]);
        x.push(val);
        println!("x[{}] = {}",x_idx,x[x_idx]);
    }
}