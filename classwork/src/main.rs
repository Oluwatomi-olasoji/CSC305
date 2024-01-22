mod how_to_hold_data_for_operations;
use how_to_hold_data_for_operations:: {derived::user_defined, derived::my_user_defined,derived::circle_and_traingle,primitive::compound, primitive::scalar};
fn main() {
    derived::lets_make_shapes();

    println!("\nSCALAR FUNCTIONS BEGIN HERE");
    scalar::run2();
    scalar::run();
   
    //compound
    println!("\nCOMPOUND FUNCTIONS BEGIN HERE");
    compound::run();
    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}",compound::reverse(pair));

    // Arrays can be automatically borrowed as slices.
     let xs: [i32; 5] = [1, 2, 3, 4, 5];
     println!("Borrow the whole array as a slice.");
     compound::analyze_slice(&xs);
 
     // Slices can point to a section of an array.
     // They are of the form [starting_index..ending_index].
     // `starting_index` is the first position in the slice.
     // `ending_index` is one more than the last position in the slice.
     println!("Borrow a section of the array as a slice.");
     compound::analyze_slice(&xs[1 .. 4]);
    
     let ts:[f64;5]= [1.0,2.0,3.0,4.0,5.0];
     println!("The array ts is {:?}",ts);
     println!("The array multiplied is {}", compound::multiplier(&ts));
     
     println!("\nHow to use match with the get method and option types");
     compound::run3();
     
     //ALIASING
     compound::run4();
}
