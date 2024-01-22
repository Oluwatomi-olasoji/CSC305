// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

pub fn run() {
    let logical: bool = true; //type annotation
    
    let a_float:f64 = 1.0; //regular annotaion
    let an_integer = 5i32; //suffix annotation 

    let default_float = 3.0; // the size here is set to the default and type is inferred
    
    let mut inferred_type = 12; //mutable variable, the type is inferred from the lin after
    inferred_type =  4294967296i64;  //note taht rust variables are immutable by default
    
    //mutable = true; //you cant change the datatype of a variable even if its mutable 

    // let mutable = true; //this will work though and is called overshaddowing  
    println!(" the value of logical is {}/na_float has a value of {}, and an_integer has the value of {}/ndefault_float ={}/n
    inferred_type ={}",logical,a_float,an_integer,default_float,inferred_type);
}

pub fn run2(){
//Integer addition
println!( "1 + 2= {}",1u32 + 2);

//Int subtraction
println!("1 - 2= {}", 1i32 - 2); //1 needs to be signed so that the resulting type can be retured correctly since its a negative number

//Scientific notation of powers
println!("1e4 is {}, -2.3e-3 is{}", 1e4, -2.5e-3);

//boolean logic
println!("true AND false is {}", true && false);
println!("true OR false is {}", true || false);
println!("NOT true is {}",!true);

// Bitwise operations
println!("0011 AND 0101 is {:b}", 0b0011u32 & 0b0101); //this means comparing each pair of corresponding bits using the logical AND operation
println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); 
println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
println!("1 << 5 is {}", 1u32 << 5);
println!("1 << 5 is {} aka {:04b} << 5 which is {:06b}", 1u32 << 5,1u32,1u32 << 5); //left shiftin 0001 (in binary) by 5 gives 10000, then conerting that back to base 10 we have 32 
println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); //once again this is converted to the binary equvalent (10000000) and right shifted
                                               // by 2 reulting in 00100000, this is then converted to decimal

//remember that when performing a left or right shift, the bits that are shifted away are deleted and the other side is padded with 0s                                              

// Use underscores to improve readability!
println!("One million is written as {}", 1_000_000u32);
}

//CASTING
pub fn run3() {
    println!("\n CASTING STARTS HERE");
    let decimal = 65.4321_f32;

    //implicit conversion like below does not work
   // let integer: u8 = decimal;
    
   //Explicit conversion
    let integer = decimal as u8;
    let character = integer as char; 

    //Limitations to conversions
     // A float cannot be directly converted to a char.
     //let character = decimal as char;
     //error[E0604]: only `u8` can be cast as `char`, not `f32`
     // FIXME ^ Comment out this line
    
     println!("Casting: {} -> {} -> {}", decimal, integer, character);

     // when casting any value to an unsigned type, T,
     // T::MAX + 1 is added or subtracted until the value
     // fits into the new type
 
     // 1000 already fits in a u16
     println!("100 as a u15 id: {}",1000 as u16);

    
    println!("1000 as a u8 is : {}", 1000 as u8);
     // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

}
//ALIASING
pub fn run4(){
 println!("\nALIASING SATRTS HERE");
 //the alias for the variable type must be written in UpperCamelCase
 // `NanoSecond`, `Inch`, and `U64` are new names 
 type NanoSecond = u64;
 type Inch = u64;
 type U64 = u64;

 // `NanoSecond` = `Inch` = `U64` = `u64`.
 let nanoseconds: NanoSecond = 5 as U64;
 let iches:Inch = 5 as U64;
 let inchz: Inch = 5 as Inch;

 // Note that type aliases *don't* provide any extra type safety, because
 // aliases are *not* new types
 println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds, inches,  nanoseconds + inches);
 println!("Inchz is {}", inchz);            
}