fn bad_fib(n: u64) -> u64{
    if n == 0 {
        return 0;
    }else if n == 1 {
        return 1;
    }else{
        return bad_fib(n-1) + bad_fib(n-2);
    }
}
//boooooii
fn main(){
    let n: u64 = 10; 
    println!("Fib of {} = {}",n,bad_fib(n));
    
    // Factorial
    println!("{}! Approx: {}, Naive: {}", n, Stirling_approximation(n), factorial(n));
    //
}

// Stirling's approximation of factorial
fn Stirling_approximation(n: u64) -> f64{
    return (n as f64 / std::f64::consts::E).powf(n as f64) * (2.0 * std::f64::consts::PI * n as f64).powf(0.5);
}
// Naive factorial
fn factorial(n: u64) -> u64{
    if n == 0 || n == 1{
        return 1;
    }
    return factorial(n-1) * n;
}

fn taylor_approx(n: u64,a: u64) -> u64{
    return 0;
}