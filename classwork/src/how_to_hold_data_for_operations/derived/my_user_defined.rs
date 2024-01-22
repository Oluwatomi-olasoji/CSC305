pub fn run(){
    //Struct items 
   // Struct Point{x:i32 , y:i32} //struct definitions dont have ;

    //p of type Point
    let p = Point {10,11};
    let px: i32 = p.x;
    println!{"p is {:?} and c in p is {}",p,px};
}