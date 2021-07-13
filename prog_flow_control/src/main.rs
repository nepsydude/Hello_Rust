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
}

