use std::mem;
pub fn multiplier(array:&[f64]) -> f64 { 
    let mut product:f64 =1f64; //needs to be declared outside the for
    for x in 0 .. array.len()  {
        product *= array[x];
    }
    product
    
}
// Tuples can be used as function arguments and as return values.
pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

pub fn run(){
    //a TUPLE WITH DIFFERENT TYPES
    let long_tuple = (1u8, 2u16,3u32,4u64,-1i8,-2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value is:{}", long_tuple.0);
    println!("Long tuple second value is:{}",long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("Tuple of tuples: {:?}",tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed.
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error
    
    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}
//ARRAYS & SLICES
//Slices are like arrays but their size is not known at compile timw
// This function borrows a slice.
pub fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}
pub fn run2() {//Fixed size array
    let xs: [i32;5] = [1,2,3,4,5];
    
    //All elements can be initialized to the same value
    let ys: [i32;20] = [0;20]; //an array of 20 zeros
    
    //Indexing starts from 0
    println!("First element of the array:{}",xs[0]);
    println!("Second element of the array is:{}", xs[0]);
    
    //'len()' returns the count of elements in the array 
    println!("Number of the elements in the array is {}",xs.len());

    //Arrays are stack allocated 
    println!("Array occupies {} bytes",mem::size_of_val(&xs));

    //Example of empty slice '&[]'
    let empty_array: [u32;0] = [];
    assert_eq!(&empty_array, &[]); //the assert_eq! is a macro that checks equality. this checks equalitY between the array we just made and an empty slice
    assert_eq!(&empty_array, &[][..]); // Same but more verbose i.e an empty slice can be written as &[] or &[][..]

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option` type which either a 'some(value)' or a 'none'. Option is an enumerated type. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
   

    for i in 0..xs.len() + 1 { //the +1 is to test what happens when it goes out of bounds
        match xs.get(i) {
            Some(xval) => println!("{}: {}",i,xval),
            None => println!("Slow down! {} is too far", i),
        }
    }
    // Out of bound indexing on array causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);
}

pub fn run3(){
    let ys:[i32;3] = [1,2,3];
    println!("The array 'ys' is {:?}",ys);
    for i in 0..ys.len() + 1{
      match ys.get(i) {
        Some(yval) => println!("Value at index {} is {}",i,yval),
        None =>  println!("Index {} is out of bounds",i),
      } //match is lowkey a return statement
    }
}


