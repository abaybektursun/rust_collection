#[macro_use] extern crate prettytable;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

use std::fmt::Write as FmtWrite;
use std::io::Write  as IoWrite;

extern crate rand;
use rand::Rng;

extern crate num_traits;
use num_traits::pow;

use std::cmp;


// Register Structure
struct REG {
    val:  u16,     // Value stored in the register
    bits: Vec<u8>, // Binary representation of the value
    size: usize    // Length of the binary represntation
}
impl REG {
    
    fn push_val(&mut self,val_in: u16) {
        if self.bits.len() == 0 {
            for x in 0..self.size{
                self.bits.push(0);
            }
        }
        
        let mut bits_str = String::new();
        write!(bits_str, "{:b}", val_in);
        
        // !DEBUG TEMP!######################
        println!("Hex Value: {:X}",val_in);
        // !DEBUG TEMP!######################
        
        for idx in 0..cmp::min(bits_str.len(), self.size) {
            let a_bit:String = bits_str.chars().nth(bits_str.len() - 1 - idx).unwrap().to_string();
            self.bits[self.size - 1 - idx] = a_bit.parse::<u8>().unwrap();
        }
        
        self.val = bits_to_u16(&self.bits);
        
        // !DEBUG TEMP!######################
        //for i in 0..self.size{
        //    println!("{}", self.bits[i]);
        //}
        // !DEBUG TEMP!######################
    }
    
    fn push_bits(&mut self, val_in: &mut Vec<u8>) {
        //std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    
    //Memory-----------------------------------------------//
    const MEM_SIZE:usize = 4096;                           //
    let mut memory:Vec<u16> = Vec::with_capacity(MEM_SIZE);//
    mem_rand(&mut memory, MEM_SIZE);                       //
    //-----------------------------------------------------//

    //Registers ----------------------------------------------------------------------------------------------|    
    let size_PC:   usize = 12;                                                        // 12 bits              |
    let size_AR:   usize = 12;                                                        // 12 bits              |
    let size_DR:   usize = 16;                                                        // 16 bits              |
    let size_AC:   usize = 16;                                                        // 16 bits              |
    let size_IR:   usize = 16;                                                        // 16 bits              |
    let size_TR:   usize = 16;                                                        // 16 bits              |
    let size_OUTR: usize = 8;                                                         // 8 bits               |
    let size_INPR: usize = 8;                                                         // 8 bits               |
    let size_E:    usize = 1;                                                         // 1 bit                |
    let mut PC   = REG{val: 0, bits: Vec::with_capacity(size_PC),   size: size_PC};   // Program Counter      |
    let mut AR   = REG{val: 0, bits: Vec::with_capacity(size_AR),   size: size_AR};   // Address Register     |
    let mut DR   = REG{val: 0, bits: Vec::with_capacity(size_DR),   size: size_DR};   // Data Register        |
    let mut AC   = REG{val: 0, bits: Vec::with_capacity(size_AC),   size: size_AC};   // Accumulator          |
    let mut IR   = REG{val: 0, bits: Vec::with_capacity(size_IR),   size: size_IR};   // Instruction Register |
    let mut TR   = REG{val: 0, bits: Vec::with_capacity(size_TR),   size: size_TR};   // Temporary Register   |
    let mut OUTR = REG{val: 0, bits: Vec::with_capacity(size_OUTR), size: size_OUTR}; // Input Register       |
    let mut INPR = REG{val: 0, bits: Vec::with_capacity(size_INPR), size: size_INPR}; // Output Register      |
    let mut E    = REG{val: 0, bits: Vec::with_capacity(size_E),    size: size_E};    // That Carry Thingy    |
    //--------------------------------------------------------------------------------------------------------|
    
    let mut PC_init   = rng.gen::<u16>();
    let mut AR_init   = rng.gen::<u16>();
    let mut DR_init   = rng.gen::<u16>();
    let mut AC_init   = rng.gen::<u16>();
    let mut IR_init   = rng.gen::<u16>();
    let mut TR_init   = rng.gen::<u16>();
    let mut OUTR_init = rng.gen::<u16>();
    let mut INPR_init = rng.gen::<u16>();
    let mut E_init    = rng.gen::<u16>();
    
    PC.push_val  (PC_init);   AR.push_val  (AR_init);
    DR.push_val  (DR_init);   AC.push_val  (AC_init);
    IR.push_val  (IR_init);   TR.push_val  (TR_init);
    DR.push_val  (DR_init);   AC.push_val  (AC_init);
    IR.push_val  (IR_init);   TR.push_val  (TR_init);
    OUTR.push_val(OUTR_init); INPR.push_val(INPR_init);
    E.push_val(0);
    
    // Create a pretty table
    let mut pretty = Table::new();
    // Add a row
    //IR AC DR PC AR M[AR] E
    pretty.add_row(row!["Register Transfer Statement", 
                        "IR", 
                        "AC", 
                        "DR", 
                        "PC", 
                        "AR", 
                        "M[AR]", 
                        "E"
                    ]);
    pretty.add_row(row!["Initial Values ", 
                        format!("{:X}",IR.val),  
                        format!("{:X}",AC.val), 
                        format!("{:X}",DR.val), 
                        format!("{:X}",PC.val), 
                        format!("{:X}",AR.val), 
                        format!("{:X}",memory[AR.val as usize]), 
                        format!("{:X}",E.val)
                    ]);

    pretty.printstd();
}

fn mem_rand(memory: &mut Vec<u16>, MEM_SIZE:usize){
    
    let mut rng = rand::thread_rng();
    for x in 0..MEM_SIZE {
        memory.push(rng.gen::<u16>());
    }
}

fn bits_to_u16(bits_in: &Vec<u8>) -> u16{
    let mut total: u16 = 0;
    for idx in 0..bits_in.len(){
        total += (bits_in[bits_in.len() - 1 - idx] as u16) * pow(2u16,idx);
    }
    return total;
}

