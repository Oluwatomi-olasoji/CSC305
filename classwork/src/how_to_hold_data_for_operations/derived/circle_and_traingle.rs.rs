//#![allow(dead_code)] //This suppresses warnings when a given declared function is  not used.
use core::cmp::Ordering; //Used dor comparison of value sizes 
use std::f32::consts::PI;

trait Shape { //this are general triats to be used in triangle and circle
    fn area(&self) -> f64;
    fn perimeter(&self)->f64;
    fn set_name(&mut self, name: &'static str);
    fn get_name(&self) -> &str;
    fn equality(&self, other: &Self) -> bool;
}

//CIRCLE
struct Circle { //creating the struct circle with the fields radius and name
    radius: f32,
    name: &'static str,
}

impl Circle {
    //default default() function. Will override derived default if any. 
    fn default() -> Self {
        Circle {
            radius: 1,
            name: "default_circle",
        }
    }
    fn new(radius: f64, name: &'static str) -> Self {
        Circle {
            radius,
            name,
        }
    }
    fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    fn get_radius(&self) -> f32 {
        self.radius
    }
}

impl Shape for Circle{ //defining the shape traits for circle
    fn area(&self) -> f64 {
        pow(self.radius,2.0) * PI
    }
    fn perimeter(&self) -> f64 {
        self.raduis * 2.0 * PI
    }
    fn set_name(&mut self, name: &'static str) {
        self.na me = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn equality(&self, other: &Self) -> String{ //a triat that checks if one shape is graeter or less than the otther
        if (self.perimeter > other.perimeter) {
            println!{"{} is greater than {}",self.name, other.name}
           // self.perimeter > other.perimeter
        }

        else if (self.perimeter < other.perimeter) {
            println!{"{} is less than {}",self.name, other.name}
            //self.perimeter < other.perimeter
        }

        else  {
            println!{"{} and  {} are equal",self.name, other.name}
           
    }
}
}

 impl PartialEq for Circle { //implementing inbuilt traits that check for equality 
    fn eq(&self, other: &Self) -> bool { 
        self.perimeter() == other.perimeter()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}


//this is a way to make a new circle ny pasisng a string slice with radiusand name, separated by commas
impl From<&'static str> for Circle {
    fn from(s: &'static str) -> Circle {
        let mut parts = s.split(',');
        let radius = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };

        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Circle{ radius, name: &name }
    }
}

//TRIANGLE
struct Triangle { //creating the struct triangle with the fields base, height, sides and name
    base: f32,
    height: f32,
    side_a: f32,
    side_b: f32,
    side_c: f32,
    name: &'static str,
}

impl Triangle { 
    fn default() -> Self {
        Triangle {
            base: 1.0,
            height: 1.0,
            side_a: 1.0,
            side_b: 1.0,
            side_c: pow(2.0,0.5),
            name: &'static str,
            name: "default_circle",
        }
    }
    fn new(base: f64, base: f32,height: f32,side_a: f32,side_b: f32, side_c: f32,name: &'static str) -> Self {
        Triangle {
            base,
            height,
            side_a,
            side_b,
            side_c,
            name: &'static str,
        }
    }
    fn set_base(&mut self, base: f32) { //this trait sets the base of the tringle
        self.base = base;
    }

    fn get_base(&self) -> f32 { //this trait gets the base of the tringle
        self.base
    }
    fn set_height(&mut self, height: f32) { //this trait sets the height of the tringle
        self.height = height;
    }
    fn height(&self) -> f32 {//this trait gets the height of the tringle
        self.height
    }

    fn get_side_a(&self) -> f32 {
        self.side_a
    }
    fn set_side_a(&mut self, side_a: f32) {
        self.side_a = side_a;
    }

    fn get_side_b(&self) -> f32 {
        self.side_b
    }
    fn set_side_b(&mut self, side_a: f32) {
        self.side_b= side_b;
    }
    fn get_side_c(&self) -> f32 {
        self.side_c
    }
    fn set_side_a(&mut self, side_a: f32) {
        self.side_c = side_c;
    }
}

impl Shape for Triangle{ //defining the shape traits for triangle
    fn area(&self) -> f64 {
       self.base * self.height * 0.5
    }
    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn equality(&self, other: &Self) -> String{ //a triat that checks if one shape is graeter or less than the otther
        if (self.perimeter > other.perimeter) {
            println!{"{} is greater than {}",self.name, other.name}
           // self.perimeter > other.perimeter
        }

        else if (self.perimeter < other.perimeter) {
            println!{"{} is less than {}",self.name, other.name}
            //self.perimeter < other.perimeter
        }

        else  {
            println!{"{} and  {} are equal",self.name, other.name}
           
    }
}
}


impl PartialEq for Triangle { //implementing inbuilt traits that check for equality 
    fn eq(&self, other: &Self) -> bool { 
        self.perimeter() == other.perimeter()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}


//this is a way to make a new traingle ny pasisng a string slice with radiusand name, separated by commas
impl From<&'static str> for Triangle {
    fn from(s: &'static str) -> Triangle {
        let mut parts = s.split(',');
        let base = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };

        let height = match parts.next() {
            Some(val) => val,
            None => "",
        };

        let side_a = match parts.next() {
            Some(val) => val,
            None => "",
        };

        let side_b = match parts.next() {
            Some(val) => val,
            None => "",
        };

        let side_c = match parts.next() {
            Some(val) => val,
            None => "",
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Triangle{ base, height, side_a, side_b, side_c, name: &name }
    }
}

pub fn lets_make_shapes() {
    //IMPLEMENTATION FOR CIRCLES BEGINS
    println!("IMPLEMENTING CIRCLES");

    let circle1 = Circle::default(); //assigns circel1 default values using the trait default
    
    //prints out the name and radius of circle1
    println!("{}", circle1.radius);
    println!("{}", circle1.name);

    let circle2 = Circle::new(3, "Circle2"); //uses the new trait defined earlies  to make a new circle
    let circle3 = Circle::from("5,Circle3");

    //Compare using equality trait defined for cicle
    circle2.equality(&circle3);

    //Compare using PartialEq
    let result = circle3.eq(&rectangle3);
    println!("the result of checking if circle2 and circle3 are equal is = {:?}", result);

    let result2 = circle2.ne(&circle3);
    println!("the result of checking if circle2 and circle3 are equal is = {:?}", result2);

    /*************************************************************888 */
    //IMPLEMENTATION FOR TRIANGLES BEGINS
    println!("IMPLEMENTING TRIANGLES");

    let triangle1 = Triangle::default(); //assigns traingle default values using the trait default
    
    //prints out the name and radius of triangle
    println!("{}", triangle1.radius);
    println!("{}", triangle1.name);

    let traingle2 = Triangle::new(14,7,9,14,11, "Triangle2"); //uses the new trait defined earlies  to make a new circle
    let traingle3 = Triangle::from("4,3,5,4,3,Triangle3");

    //Compare using equality trait defined for traingle
    traingle2.equality(&traingle3);

    //Compare using PartialEq
    let result = traingle2.eq(&traingle3);
    println!("the result of checking if triangle2 and traingle3 are equal is = {:?}", result);

    let result2 = triangle2.ne(&traingle3);
    println!("the result of checking if traingle2 and traingle3 are equal is = {:?}", result2);

    //COMPARING THE PERIMETERS OF A TRIANGLE AND A CIRCLE
    let result2 = traingle2.eq(&circle2);
    println!("the result of checking if triangle2 and cicle2 are equal in perimeter is = {:?}", result2);
}
