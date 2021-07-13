// Conditional Executions

fn main() {
    let x = 4;

    if x + 1 != 3 {
        println!("x + 1 is NOT 3!");
    }

    // Multiple conditions examples

    let x = 7;
    let y = 7;

    if x > y {
        println!("x is greater than y");
    } else {
        if x < y {
            println!("x is less than y");
        } else {
            println!("x is equal to y");
        }
    }

    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal to y");
    }


    // Loop examples

    let mut count = 0;

    let result = loop {
        if count == 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };

    println!("After the loop!");
    println!("result is {}", result);
    let mut count = 0;
    let letters = ['a', 'b', 'c'];

    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;
    }
    let message = ['h', 'e', 'l', 'l', 'o'];

    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'e' {
            break;
        }
    }

    for number in 10..5 {
        println!("number is {}", number);
    }

    // Nested loop example

    let mut matrix = [[1, 2, 3],
                      [4 ,5 ,6],
                      [7, 8, 9]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
           *num += 10;
           print!("{}\t", num);
        }
        println!();
    }
 // Tests , lets check loop

    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    max = numbers[0]; // initialize
    min = numbers[0];
    mean = 0.0;

    for &num in numbers.iter(){ // iterate through numbers
        if num > max {          // checks if current number is maximum
            max = num;
        } else if num < min {   // checks min

            min = num;
        }
        mean += num as f64   //adds current number to mean variable

    }

    mean /= numbers.len() as f64;


    assert_eq!(max,56);
    assert_eq!(min,-18);
    assert_eq!(mean,12.5);
    println!("Tests passed!");

// Shadowing variable


    let planet = "Earth";
    {
        println!("planet is {}", planet);
        let mut planet = 4;
        println!("planet is {}", planet);
    }
    println!("planet is {}", planet);

    // stack and heap memory

    // strings are stored in the heap others in the stack


    // string allocation
    //from function is associated with String
    // :: is path operator- allows access to from function
    let mut message = String::from("Earth");
    println!("message is {}", message);
    message.push_str(" is home.");
    println!("message is {}", message);
    // check struct string rust documentation

    
    // Heap is significantly big than stack but not infinite
    // we need to clear now and then to not run ot of space
    // Memory management with ownership
    let outer_planet: i32;
    {
        let mut inner_planet = 1;
        outer_planet = inner_planet;
        inner_planet += 1;
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
    
    // referencing borrowing
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);


}

    


// others include dangling , slicing,  and slices as function parameter
// slices are in bytes
// ranges indices must occur at calid utf8 character

/* [1,2,3,4] are considered slice when inside a string.println!

Write a function to remove leading and trailing spaces
from  a string */

