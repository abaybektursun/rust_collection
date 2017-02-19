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
        if self.bits.len() == 0{
            for x in 0..self.size{
                self.bits.push(0);
            }
        }
        else{
            for x in 0..self.size{
                self.bits[x] = 0;
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
    
    fn push_bits(&mut self,  bits_in: &Vec<u8>) {
    
        if self.bits.len() == 0{
            for x in 0..self.size{
                self.bits.push(0);
            }
        }
        else{
            for x in 0..self.size{
                self.bits[x] = 0;
            }
        }
        
        // !DEBUG TEMP!####################################################
        //println!("push_bits Min: {}",cmp::min(bits_in.len(), self.size));
        //println!("size: {}",self.size);
        // !DEBUG TEMP!####################################################
        
        for idx in 0..cmp::min(bits_in.len(), self.size) {
            self.bits[self.size - 1 - idx] = bits_in[bits_in.len() - 1 - idx];
        }
        
        self.val = bits_to_u16(&self.bits);
        
        // !DEBUG TEMP!######################
        //println!("val updated!");
        // !DEBUG TEMP!######################
        
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
    
    // Initialize registers randomly -----//
    let mut PC_init   = rng.gen::<u16>(); //
    let mut AR_init   = rng.gen::<u16>(); //
    let mut DR_init   = rng.gen::<u16>(); //
    let mut AC_init   = rng.gen::<u16>(); //
    let mut IR_init   = rng.gen::<u16>(); //
    let mut TR_init   = rng.gen::<u16>(); //
    let mut OUTR_init = rng.gen::<u16>(); //
    let mut INPR_init = rng.gen::<u16>(); //
    let mut E_init    = rng.gen::<u16>(); //
    //- - - - - - - - - - - - - - - - - - //
    PC.push_val  (PC_init);               //
    AR.push_val  (AR_init);               //
    DR.push_val  (DR_init);               //
    AC.push_val  (AC_init);               //
    IR.push_val  (IR_init);               //
    TR.push_val  (TR_init);               //
    OUTR.push_val(OUTR_init);             //
    INPR.push_val(INPR_init);             //
    E.push_val   (0);                     //
    //------------------------------------//
    
    // Create a pretty table
    let mut pretty = Table::new();
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
                    
    mano_automata(&mut memory, &mut pretty, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
    

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
    
    // !DEBUG TEMP!######################
    //println!("length: {}",bits_in.len());
    // !DEBUG TEMP!######################
    
    for idx in 0..bits_in.len(){
    
        // !DEBUG TEMP!###################################
        //println!("Index: {}", idx);
        //println!("2 to {} power: {}", idx, pow(2u16,idx));
        // !DEBUG TEMP!###################################
        total += (bits_in[bits_in.len() - 1 - idx] as u16) * pow(2u16,idx);
    }
    return total;
}

fn mano_automata(
    memory: &mut Vec<u16>, mut pretty: &mut Table, 
    mut IR: &mut REG, 
    mut AC: &mut REG, 
    mut DR: &mut REG, 
    mut PC: &mut REG, 
    mut AR: &mut REG, 
    mut E:  &mut REG
){
    AR.push_bits(& PC.bits);
    pretty.add_row(row!["T[0]: AR ← PC ", 
                        format!("{:X}",IR.val),  
                        format!("{:X}",AC.val), 
                        format!("{:X}",DR.val), 
                        format!("{:X}",PC.val), 
                        format!("{:X}",AR.val), 
                        format!("{:X}",memory[AR.val as usize]), 
                        format!("{:X}",E.val)
                    ]);
    
    T_1(& memory, &mut pretty, &mut IR, & AC, & DR, &mut PC, &mut AR, &mut E);
}

fn T_1(
    memory: & Vec<u16>, mut pretty: &mut Table, 
    IR: &mut REG, 
    AC: & REG, 
    DR: & REG, 
    PC: &mut REG, 
mut AR: &mut REG, 
mut E:  &mut REG
    )
{
    IR.push_val(memory[AR.val as usize]);
    let PC_local = PC.val;
    PC.push_val(PC_local + 1);
    
    pretty.add_row(row!["T[1]: IR ← M[AR]; PC++ ", 
                        format!("{:X}",IR.val),  
                        format!("{:X}",AC.val), 
                        format!("{:X}",DR.val), 
                        format!("{:X}",PC.val), 
                        format!("{:X}",AR.val), 
                        format!("{:X}",memory[AR.val as usize]), 
                        format!("{:X}",E.val)
                    ]);
    
    T_2(& memory, &mut pretty, & IR, & AC, & DR, & PC, &mut AR, &mut E);
}

fn T_2(
    memory: & Vec<u16>, pretty: &mut Table, 
    IR: &    REG, 
    AC: &    REG, 
    DR: &    REG, 
    PC: &    REG, 
    AR: &mut REG, 
    E:  &mut REG
    )
{
    AR.push_val(IR.val);
     E.push_val(IR.bits[15] as u16);
     
    // Decode
    let action_code: u16 = bits_to_u16(& vec![IR.bits[12], IR.bits[13], IR.bits[14]] );
    
    // !DEBUG TEMP!##################################################
    //println!("Elems:     {}{}{}", IR.bits[12], IR.bits[13], IR.bits[14]);
    //println!("Converted: {}"    , action_code);
    // !DEBUG TEMP!##################################################

    pretty.add_row(row!["T[2]: AR ← IR(0-11) ", 
                        format!("{:X}",IR.val),  
                        format!("{:X}",AC.val), 
                        format!("{:X}",DR.val), 
                        format!("{:X}",PC.val), 
                        format!("{:X}",AR.val), 
                        format!("{:X}",memory[AR.val as usize]), 
                        format!("{:X}",E.val)
                    ]);
}


