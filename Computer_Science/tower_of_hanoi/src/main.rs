use std::process;

fn pop_error(){
    println!( "Error peforming pop(). Exit with code 2" );
    process::exit(2);
}
    
fn hanoi(n, source, target, auxiliary){
    
}
    

fn main(){
    let mut source: Vec<u32> = Vec::new();
    let mut target: Vec<u32> = Vec::new();
    let mut extra:  Vec<u32> = Vec::new();
    
    let num_of_plates: u32 = 20;

    for a_plate in 0..num_of_plates{
        println!("{}",num_of_plates - a_plate);
        source.push(num_of_plates - a_plate);
    }
    
    /*let popped = source.pop();
    match popped{
        Some(popped) => println!( "{}",popped ),
        None         =>  pop_error()
    }*/
}