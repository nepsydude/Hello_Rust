fn main() {
    let mut x = 10; // mutable variable
    
    println!("x is {}",x);
    x = 20; // unmuted variable
    println!("x is {}",x); // both lines are printed due to mutable variable

    /* naming conventions 
        rust-lang.github.io/api-guidelines/naming.html
    */

    let y: i32 = -10;
    println!("y is {}",y);

    // floating point numbers

    let z:f32 = 3.1232939484934839;
    println!("z is {}",z);

    // arithmetic operations

    let a = 10;
    let b = 3;
    let c = a+b;
    let d = a%b; //modulo
    println!("c is {}",c);
    println!("d is {}",d);


}



