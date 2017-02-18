

fn main() {
	const MEM_SIZE:usize = 4096;
	let mut memory:Vec<i16> = Vec::with_capacity(MEM_SIZE);
	mem_rand(&mut memory, MEM_SIZE);
	print!("{}", memory[0]);   
}

fn mem_rand(memory: &mut Vec<i16>, MEM_SIZE:usize){
	for x in 0..MEM_SIZE {
		if x%100 == 0{
		println!("{}",x);
		}
	}	
}
