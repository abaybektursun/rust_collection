extern crate dataplotlib;
#[macro_use] extern crate prettytable;

use std::io;
use std::ops::Div;
use std::f64;
use std::str::FromStr;
use std::env;
use dataplotlib::util::{linspace, zip2};
use dataplotlib::plotbuilder::PlotBuilder2D;
use dataplotlib::plotter::Plotter;

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

fn main() {

    let x_approx: f64 = 1.0;

    let iters: f64 = 100.0;
    
    let mut x = 0.0f64;
    let mut y = 1.0f64;
    let mut dydx = y;
    let h = x_approx / iters;
    let mut tru_val = 1.0f64;
    
    let mut tru_vec = vec![tru_val];
    let mut approx_vec = vec![y];
    let mut x_vec = vec![x];
    
    // Create the table
    let mut prettyTable = Table::new();
        
    // Add headers
    prettyTable.add_row(row!["x", "Linear Approximation", "Calculated Value", "Difference"]);
    
    let mut difference: f64 = 0.0;
    let mut difference_vec = vec![difference];
    
    while x <= x_approx{
        y = y + h * dydx;
        dydx = y;
        x += h;
        
        tru_val = f64::consts::E.powf(x);
        difference = tru_val - y;
        difference = difference.abs();
        //println!("f({}) Linear Approximation: {}\t|\tTrue Value: {}", x, y, tru_val);
        prettyTable.add_row(row![format!("{:.*}", 4, x), format!("{:.*}", 4, y), format!("{:.*}", 4, tru_val), format!("{:.*}", 4, difference)]);
        
        difference_vec.push(difference);
        tru_vec.push(tru_val);
        approx_vec.push(y); 
        x_vec.push(x);
    }
    
    
    let diff = f64::consts::E - y;
    
    
    let xy_approx = zip2(&approx_vec, &x_vec);

    let xy_true = zip2(&tru_vec, &x_vec);

    // Creates a new plot builder
    let mut pb = PlotBuilder2D::new();

    // Adds the sin plot and the linear plot with custom colors
    pb.add_color_xy(xy_approx, [1.0, 0.0, 0.0, 1.0]);
    pb.add_color_xy(xy_true, [0.0, 0.0, 1.0, 1.0]);

    let mut plt = Plotter::new();
    plt.plot2d(pb);
    plt.join();
    
    println!("-----------------------------------------------------------------------------\n\n");

    prettyTable.printstd();
    
    println!("\nFinal Error of Approximation of E: {}\n",diff.abs());
    
    let singleStp = 1.0 + 1.0 * 1.0;
    let singleStpDiff = f64::consts::E - singleStp;
    
    println!("\nSingle Step Approximation Error: {}",singleStpDiff.abs());
    
}