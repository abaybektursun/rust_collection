#[macro_use] extern crate prettytable;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

use std::fmt::Write as FmtWrite;
use std::io::Write  as IoWrite;
use std::u16;

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
        //println!("Hex Value: {:X}",val_in);
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
    let mut E_init    = rng.gen       (); //
    //- - - - - - - - - - - - - - - - - - //
    PC.push_val  (PC_init);               //
    AR.push_val  (AR_init);               //
    DR.push_val  (DR_init);               //
    AC.push_val  (AC_init);               //
    IR.push_val  (IR_init);               //
    TR.push_val  (TR_init);               //
    OUTR.push_val(OUTR_init);             //
    INPR.push_val(INPR_init);             //
    E.push_val   (E_init);                //
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
    let mut epoch: u8 = 5;
    mano_automata(&mut memory, &mut pretty, epoch, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
    

    pretty.printstd();
    
   // !DEBUG TEMP!######################
   //let test16_ : u16 = 0xFFFF;
   //let test16_2: u16 = 0xFFFF;
   //let test32_ : u32 = test16_ as u32 + test16_2 as u32;
   //println!("test u 16: {:X}",test16_);
   //println!("test u 32: {:X}",test32_ as u16);
   // !DEBUG TEMP!######################
}

// Random memory initialization
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
    mut memory: &mut Vec<u16>, mut pretty: &mut Table, mut epoch: u8,
    mut IR: &mut REG, 
    mut AC: &mut REG, 
    mut DR: &mut REG, 
    mut PC: &mut REG, 
    mut AR: &mut REG, 
    mut E:  &mut REG
){
    if epoch > 0 {
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
                        
        T_1(&mut memory, &mut pretty, epoch, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
    }
    epoch -= 1;
}

fn T_1(
    mut memory: &mut Vec<u16>, mut pretty: &mut Table, mut epoch: u8,
    mut IR: &mut REG, 
    mut AC: &mut REG, 
    mut DR: &mut REG, 
    mut PC: &mut REG, 
    mut AR: &mut REG, 
    mut E:  &mut REG
    )
{
    IR.push_val(memory[AR.val as usize]);
    let PC_local = PC.val;
    // TODO! Handle overflow?
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
    
    T_2(&mut memory, &mut pretty, epoch, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
}

fn T_2(
    mut memory: &mut Vec<u16>, mut pretty: &mut Table, mut epoch: u8,
    mut IR: &mut REG, 
    mut AC: &mut REG, 
    mut DR: &mut REG, 
    mut PC: &mut REG, 
    mut AR: &mut REG, 
    mut E:  &mut REG
)
{
    AR.push_val(IR.val);
    let I = IR.bits[15];
     
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
                    
    T_3(&mut memory, &mut pretty, epoch, I, action_code, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
}

fn T_3(
    mut memory: &mut Vec<u16>, mut pretty: &mut Table, mut epoch: u8, I: u8, action_code: u16, 
    mut IR: &mut REG, 
    mut AC: &mut REG, 
    mut DR: &mut REG, 
    mut PC: &mut REG, 
    mut AR: &mut REG, 
    mut E:  &mut REG
)
{
    if action_code == 0x7{
        if I == 1{
            
        }
        else if I == 0 {
            
        }
        else {
            // ERROR 2F: VALUE OF I IS NON BINARY
            panic!("💔 ERROR 2F");
        }
    }
    else{
        if I == 1{
            let AR_local = AR.val;
            AR.push_val(memory[AR_local as usize]);
            pretty.add_row(row!["T[3]: AR ← M[AR] ", 
                        format!("{:X}",IR.val),  
                        format!("{:X}",AC.val), 
                        format!("{:X}",DR.val), 
                        format!("{:X}",PC.val), 
                        format!("{:X}",AR.val), 
                        format!("{:X}",memory[AR.val as usize]), 
                        format!("{:X}",E.val)
                    ]);
           T_4_MEM(&mut memory, &mut pretty, epoch, I, action_code, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
        }
        else if I == 0 {
            pretty.add_row(row!["T[3]: Idle ", 
                        format!("{:X}",IR.val),  
                        format!("{:X}",AC.val), 
                        format!("{:X}",DR.val), 
                        format!("{:X}",PC.val), 
                        format!("{:X}",AR.val), 
                        format!("{:X}",memory[AR.val as usize]), 
                        format!("{:X}",E.val)
                    ]);
            T_4_MEM(&mut memory, &mut pretty, epoch, I, action_code, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
        }
        else {
            // ERROR 2F: VALUE OF I IS NON BINARY
            panic!("💔 ERROR 2F")
        }
    }
}

// Memory Reference
fn T_4_MEM(
    mut memory: &mut Vec<u16>, mut pretty: &mut Table, mut epoch: u8, I: u8, action_code: u16,
    mut IR: &mut REG, 
    mut AC: &mut REG, 
    mut DR: &mut REG, 
    mut PC: &mut REG, 
    mut AR: &mut REG, 
    mut E:  &mut REG
)
{
    match action_code {
        0x0 => { //AND       
            DR.push_val(memory[AR.val as usize]);
            pretty.add_row(row!["T[4]: DR ← M[AR] ", 
                    format!("{:X}",IR.val),  
                    format!("{:X}",AC.val), 
                    format!("{:X}",DR.val), 
                    format!("{:X}",PC.val), 
                    format!("{:X}",AR.val), 
                    format!("{:X}",memory[AR.val as usize]), 
                    format!("{:X}",E.val)
                ]);
            T_5_AND(&mut memory, &mut pretty, epoch, I, action_code, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
        },
        0x1 => { //ADD       
            DR.push_val(memory[AR.val as usize]);
            pretty.add_row(row!["T[4]: DR ← M[AR] ", 
                    format!("{:X}",IR.val),  
                    format!("{:X}",AC.val), 
                    format!("{:X}",DR.val), 
                    format!("{:X}",PC.val), 
                    format!("{:X}",AR.val), 
                    format!("{:X}",memory[AR.val as usize]), 
                    format!("{:X}",E.val)
                ]);
            T_5_ADD(&mut memory, &mut pretty, epoch, I, action_code, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
        },
        0x2 => { //LDA       
            DR.push_val(memory[AR.val as usize]);
            pretty.add_row(row!["T[4]: DR ← M[AR] ", 
                    format!("{:X}",IR.val),  
                    format!("{:X}",AC.val), 
                    format!("{:X}",DR.val), 
                    format!("{:X}",PC.val), 
                    format!("{:X}",AR.val), 
                    format!("{:X}",memory[AR.val as usize]), 
                    format!("{:X}",E.val)
                ]);
            T_5_LDA(&mut memory, &mut pretty, epoch, I, action_code, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
        },
        0x3 => { //STA       
            memory[AR.val as usize] = AC.val;
            pretty.add_row(row!["T[4]: M[AR] ← AC", 
                    format!("{:X}",IR.val),  
                    format!("{:X}",AC.val), 
                    format!("{:X}",DR.val), 
                    format!("{:X}",PC.val), 
                    format!("{:X}",AR.val), 
                    format!("{:X}",memory[AR.val as usize]), 
                    format!("{:X}",E.val)
                ]);
            mano_automata(&mut memory, &mut pretty, epoch, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
        },
        0x4 => { //BUN       
            PC.push_val(AR.val);
            pretty.add_row(row!["T[4]: PC ← AR", 
                    format!("{:X}",IR.val),  
                    format!("{:X}",AC.val), 
                    format!("{:X}",DR.val), 
                    format!("{:X}",PC.val), 
                    format!("{:X}",AR.val), 
                    format!("{:X}",memory[AR.val as usize]), 
                    format!("{:X}",E.val)
                ]);
            mano_automata(&mut memory, &mut pretty, epoch, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
        },
        0x5 => { //BSA       
            memory[AR.val as usize] = PC.val;
            let AR_local = AR.val;
            // TODO! Handle overflow?
            AR.push_val(AR_local + 1);
            pretty.add_row(row!["T[4]: M[AR] ← PC; AR++", 
                    format!("{:X}",IR.val),  
                    format!("{:X}",AC.val), 
                    format!("{:X}",DR.val), 
                    format!("{:X}",PC.val), 
                    format!("{:X}",AR.val), 
                    format!("{:X}",memory[AR.val as usize]), 
                    format!("{:X}",E.val)
                ]);
            T_5_BSA(&mut memory, &mut pretty, epoch, I, action_code, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
        },
        0x6 => { //ISZ 
            DR.push_val(memory[AR.val as usize]);
            pretty.add_row(row!["T[4]: DR ← M[AR] ", 
                    format!("{:X}",IR.val),  
                    format!("{:X}",AC.val), 
                    format!("{:X}",DR.val), 
                    format!("{:X}",PC.val), 
                    format!("{:X}",AR.val), 
                    format!("{:X}",memory[AR.val as usize]), 
                    format!("{:X}",E.val)
                ]);
            T_5_ISZ(&mut memory, &mut pretty, epoch, I, action_code, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
        },
         _ =>
            // ERROR 3F: INAPPROPRIATE T_4_MEM CALL
            // action_code IS OUT OF RANGE (0-6)
            panic!("💔 ERROR 3F")
    }
}


// Registry Reference
fn T_4_REG(
    mut memory: &mut Vec<u16>, mut pretty: &mut Table, mut epoch: u8, I: u8, action_code: u16,
    mut IR: &mut REG, 
    mut AC: &mut REG, 
    mut DR: &mut REG, 
    mut PC: &mut REG, 
    mut AR: &mut REG, 
    mut E:  &mut REG
)
{
    
}

fn T_5_AND(
    mut memory: &mut Vec<u16>, mut pretty: &mut Table, mut epoch: u8, I: u8, action_code: u16,
    mut IR: &mut REG, 
    mut AC: &mut REG, 
    mut DR: &mut REG, 
    mut PC: &mut REG, 
    mut AR: &mut REG, 
    mut E:  &mut REG
)
{
    // Bitwise AND
    let AC_new = AC.val & DR.val;
    AC.push_val(AC_new);
    
    pretty.add_row(row!["T[5]: AC ← AC ^ DR ", 
                format!("{:X}",IR.val),  
                format!("{:X}",AC.val), 
                format!("{:X}",DR.val), 
                format!("{:X}",PC.val), 
                format!("{:X}",AR.val), 
                format!("{:X}",memory[AR.val as usize]), 
                format!("{:X}",E.val)
            ]);
            
    mano_automata(&mut memory, &mut pretty, epoch, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
    
}

fn T_5_ADD(
    mut memory: &mut Vec<u16>, mut pretty: &mut Table, mut epoch: u8, I: u8, action_code: u16,
    mut IR: &mut REG, 
    mut AC: &mut REG, 
    mut DR: &mut REG, 
    mut PC: &mut REG, 
    mut AR: &mut REG, 
    mut E:  &mut REG 
)
{
    let AC_new: u32 = AC.val as u32 + DR.val as u32 ;
    
    // Make E = 1 in case of overflow
    if AC_new > <u16>::max_value() as u32 {
        E.push_val(1);
    }
    else { E.push_val(0); }
    
    // Cast below trims out the left 4 bits
    AC.push_val(AC_new as u16);
    
    pretty.add_row(row!["T[5]: AC ← AC + DR ", 
                format!("{:X}",IR.val),  
                format!("{:X}",AC.val), 
                format!("{:X}",DR.val), 
                format!("{:X}",PC.val), 
                format!("{:X}",AR.val), 
                format!("{:X}",memory[AR.val as usize]), 
                format!("{:X}",E.val)
            ]);
    mano_automata(&mut memory, &mut pretty, epoch, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
}

fn T_5_LDA(
    mut memory: &mut Vec<u16>, mut pretty: &mut Table, mut epoch: u8, I: u8, action_code: u16,
    mut IR: &mut REG, 
    mut AC: &mut REG, 
    mut DR: &mut REG, 
    mut PC: &mut REG, 
    mut AR: &mut REG, 
    mut E:  &mut REG 
)
{
    AC.push_val(DR.val);
    
    pretty.add_row(row!["T[5]: AC ← DR ", 
                format!("{:X}",IR.val),  
                format!("{:X}",AC.val), 
                format!("{:X}",DR.val), 
                format!("{:X}",PC.val), 
                format!("{:X}",AR.val), 
                format!("{:X}",memory[AR.val as usize]), 
                format!("{:X}",E.val)
            ]);
    mano_automata(&mut memory, &mut pretty, epoch, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
}

fn T_5_BSA(
    mut memory: &mut Vec<u16>, mut pretty: &mut Table, mut epoch: u8, I: u8, action_code: u16,
    mut IR: &mut REG, 
    mut AC: &mut REG, 
    mut DR: &mut REG, 
    mut PC: &mut REG, 
    mut AR: &mut REG, 
    mut E:  &mut REG 
)
{
    PC.push_val(AR.val);
    
    pretty.add_row(row!["T[5]: PC ← AR ", 
                format!("{:X}",IR.val),  
                format!("{:X}",AC.val), 
                format!("{:X}",DR.val), 
                format!("{:X}",PC.val), 
                format!("{:X}",AR.val), 
                format!("{:X}",memory[AR.val as usize]), 
                format!("{:X}",E.val)
            ]);
    mano_automata(&mut memory, &mut pretty, epoch, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
}

fn T_5_ISZ(
    mut memory: &mut Vec<u16>, mut pretty: &mut Table, mut epoch: u8, I: u8, action_code: u16,
    mut IR: &mut REG, 
    mut AC: &mut REG, 
    mut DR: &mut REG, 
    mut PC: &mut REG, 
    mut AR: &mut REG, 
    mut E:  &mut REG 
)
{
    let DR_local = DR.val;
    // TODO! Handle overflow?
    DR.push_val(DR_local + 1);
    
    pretty.add_row(row!["T[5]: DR++ ", 
                format!("{:X}",IR.val),  
                format!("{:X}",AC.val), 
                format!("{:X}",DR.val), 
                format!("{:X}",PC.val), 
                format!("{:X}",AR.val), 
                format!("{:X}",memory[AR.val as usize]), 
                format!("{:X}",E.val)
            ]);
    T_6_ISZ(&mut memory, &mut pretty, epoch, I, action_code, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
}


fn T_6_ISZ(
    mut memory: &mut Vec<u16>, mut pretty: &mut Table, mut epoch: u8, I: u8, action_code: u16,
    mut IR: &mut REG, 
    mut AC: &mut REG, 
    mut DR: &mut REG, 
    mut PC: &mut REG, 
    mut AR: &mut REG, 
    mut E:  &mut REG 
)
{
    let mut step_str = "";
    if DR.val == 0 {
        let PC_local = PC.val;
        // TODO! Handle overflow?
        PC.push_val(PC_local + 1);
        step_str = "T[6]: PC++ (DR IS 0)";
    }
    else{
        memory[AR.val as usize] = DR.val;
        step_str = "T[6]: M[AR] ← DR ";
    }
    
    pretty.add_row(row![step_str, 
                format!("{:X}",IR.val),  
                format!("{:X}",AC.val), 
                format!("{:X}",DR.val), 
                format!("{:X}",PC.val), 
                format!("{:X}",AR.val), 
                format!("{:X}",memory[AR.val as usize]), 
                format!("{:X}",E.val)
            ]);
    mano_automata(&mut memory, &mut pretty, epoch, &mut IR, &mut AC, &mut DR, &mut PC, &mut AR, &mut E);
}
