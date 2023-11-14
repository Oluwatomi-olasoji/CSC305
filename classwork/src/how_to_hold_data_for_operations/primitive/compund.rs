
fn multiplier(array:&[f64]) -> f64 {
    let mut product:f64 =1f64; //needs to be declared outside the for
    for x in 0 .. array.len()  {
        product *= array[x];
    }
    product
    
}
// Tuples can be used as function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

fn run(){
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
    
}