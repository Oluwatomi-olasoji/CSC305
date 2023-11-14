//greeting function simply returns a string 
pub fn default_greeting()-> String {
    let message = String:: from("Hi:");
    message //this returns message. when you make a statement without a ';' rust knows you are trying to make a return statement
}
pub fn default_greeting2()->String {
    let message: String = String:: from("How are you doing?");
    return message; //this is another way to return a message
}