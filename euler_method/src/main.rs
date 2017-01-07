extern crate dataplotlib;

use std::io;
use std::ops::Div;
use std::f32;
use std::str::FromStr;
use std::env;
use dataplotlib::util::{linspace, zip2};
use dataplotlib::plotbuilder::PlotBuilder2D;
use dataplotlib::plotter::Plotter;

fn main() {

    let x_approx: f32 = 1.0;

    let iters: f32 = 10000.0;
    
    let mut x = 0.0f32;
    let mut y = 1.0f32;
    let mut dydx = y;
    let h = x_approx / iters;
    let mut tru_val = 1.0f32;
    
    let mut tru_vec = vec![tru_val];
    let mut approx_ve = vec![y];
    let mut x_vec = vec![x];
    
    while x <= x_approx{
        y = y + h * dydx;
        dydx = y;
        x += h;
        
        tru_val = f32::consts::E.powf(x);
        println!("f({}) Linear Approximation: {}\t|\tTrue Value: {}", x, y, tru_val);
        
        tru_vec.push(tru_val);
        approx_ve.push(y); 
        x_vec.push(x);
    }
    
    println!("-----------------------------------------------------------------------------\n");
    
    let diff = f32::consts::E - y;
    
    println!("Final Error of Approximation E: {}",diff.abs());
    
    
}