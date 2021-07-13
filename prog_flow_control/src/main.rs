// Conditional Executions

fn main() {
    let x = 4;

    if x + 1 != 3 {
        println!("x + 1 is NOT 3!");
    }

    // Multiple conditions examples
    
    let x = 3;
    let y = 5;

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
}
