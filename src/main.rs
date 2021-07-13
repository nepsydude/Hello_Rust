
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

    // Bitwise operation 

    let mut value = 0b1111_0101u8;
    println!("value is {}",value);
    println!("value is {:08b}",value);

    value = !value; // bits inversion
    println!("value is {:08b}",value);

    value = value & 0b1111_0111; // bitwise and operator
    println!("value is {:08b}",value);

    value = value | 0b1111_0111; // bitwise or operator
    println!("value is {:08b}",value);

    value = value ^ 0b0101_0101; // bitwise X-or operator
    println!("value is {:08b}",value);

    value = value << 4; // bitwise left shift operator
    println!("value is {:08b}",value);

    value = value  >> 2; // bitwise right shift by 2 bits operator
    println!("value is {:08b}",value);

    // test empty commit

    // boolean data types and operations

    let ab = true;
    let ac = false;
    println!("ab is {} and ac is {}", ab, ac);
    println!("NOT ab is {}", !ab);
    println!("ab AND ac is {}", ab & ac);
    println!("ab OR ac is {}", ab | ac);
    println!("ab XOR ac is {}", ab ^ ac);

    // comparision operators 

    let ax = 1;
    let bx = 2;
    println!("ax is {} and b is {}", ax, bx);
    println!("ax EQUAL TO b is {}", ax == bx);
    println!("ax NOT EQUAL TO b is {}", ax != bx);
    println!("ax GREATER THAN b is {}", ax > bx);
    println!("ax GREATER THAN OR EQUAL TO b is {}", ax >= bx);
    println!("ax LESS THAN bx is {}", ax < bx);
    println!("ax LESS THAN OR EQUAL TO bx is {}", ax <= bx);

    // character data types

    let letter = 'a';
    let number= '1';
    let finger = '\u{261D}';

    println!("{}\n{}\n{}", letter,number, finger);

    let p = 13;
    let q = 2.3;
    let r = 120.0;

    let avergae = (p as f64 +q+r as f64)/3.0;
    assert_eq!(avergae,45.1);
    println!("Test Passed");

    // Array

    let mut letters = ['a', 'b', 'c', 'd'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first letter is {}", first_letter);

    // multidimensional arrays

    let parking = [[1,2,3],[4,5,6]];
    let nummer = parking[0][1];
    println!("number is {}",number);

    // tuples

    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first_item is {}", first_item);

    let (a, b, c) = stuff;
    println!("b is {}", b);


    // functions
    
    say_hello();
    say_hello();
    let x = 1;
    let y = 2;
    say_the_sum(x, y);
    say_a_number(x as i32);

    let result = square(13);
    println!("result is {:?}", result);

}   

fn say_hello() {
    println!("Hello!");
    say_a_number(13);
}

// parameters in paranthesis
fn say_a_number(number: i32) {
    println!("number is {}", number);
}


fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("sum is {}", sum);
}

fn square(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x * x);
    println!("End of function");
    
}






