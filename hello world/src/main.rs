
mod greetings; //gives asses to the file greetings and in that file we have files from the folder greetings
use greetings:: {english,yoruba,hausa}; // same as "use greeting::*" since this gives access to everyrthing in the greetings file
//use greetings::default_greeting; 
//use greetings::default_greeting2; //use is used to import functions 
//if you wish to import all functions from the module greetings, this also works: use greetings::*
extern crate hello_world_lib; //this gives me access to an external crate. its is external because it is outside src 


fn main() { 
    println!("Hello, world!");
    println!("My first greeting is {}, and my second greeting is {}", english::default_greeting(), english::default_greeting2());
    println!("i can aslo greet you like this '{}', and this '{}'/n rus",yoruba::yoruba(),hausa::hausa());
    println!("yo! this is a {}", hello_world_lib::greeting_from_lib());

    //binary crates are execuatble codes and they have a main function which is the first called by an operating system
    //models are equivalant to files 
    //an expression is a statement that can be evaluated and used to give a value to a variable e.g 2, "helen" which is a literal expression, a + b
}
