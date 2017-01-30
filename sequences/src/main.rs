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
    let n: u64 = 45; 
    println!("Fib of {} = {}",n,bad_fib(n));
}