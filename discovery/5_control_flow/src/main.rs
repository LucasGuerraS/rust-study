/*
Define a `color_to_number` function that accepts a 'color'
parameter (a string). Use if, else if, and else
statements to return a corresponding numeric value based
on the following rules:
1. If the color is "red", return 1.
2. If the color is "green", return 2.
3. If the color is "blue", return 3.
4. If the color is any other string, return 0.

Refactor the function above to use the `match` statement
instead of if, else if, and else.

Define a `factorial` function that calculates the
factorial of a number. The factorial is the product
of multiplying a number by every incremental
number leading up to it, starting from 1.

Examples:
The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
factorial(5) should return 120.

The factorial of 4 is 4 * 3 * 2 * 1 = 24
factorial(4) should return 24.

Implement two solutions/functions for the problem.
The first solution should not use recursion.
The second solution should use recursion.
*/

fn main() {
    let blue = color_to_number("blue");
    let pink = color_to_number("pink");
    println!("{blue} {pink}");
    println!("5 factorial is {:?}", factorial_non_recursion(5));
    println!("4 factorial is {:?}", factorial_non_recursion(4));
    println!("5 factorial with recursion is {:?}", factorial_recursion(5));
    println!("4 factorial with recursion is {:?}", factorial_recursion(4));
    println!("1 factorial with recursion is {:?}", factorial_recursion(1));
    examples();
}

fn factorial_non_recursion(mut num: i32) -> i32 {
    let mut fact = num;
    while num > 1 {
        fact = fact * (num - 1);
        num -= 1;
    }
    return fact;
}

fn factorial_recursion(num: i32) -> i32 {
    if num <= 1 {
        return num;
    } else {
        num * factorial_recursion(num - 1)
    }
}

fn color_to_number(color: &str) -> i8 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn examples() {
    // match statement example with syntax variations
    let evaluation: bool = true;

    match evaluation {
        true => {
            println!("The value is true");
        }
        false => {
            println!("The value is false");
        }
    }

    let value = match evaluation {
        true => 40,
        false => 20,
    };

    println!("{value}");

    let foo = "Foo";

    match foo {
        "Oi" => println!("Oi"),
        "Hello" => println!("Hello"),
        _ => println!("Bye"),
    }

    let mut seconds = 5;

    loop {
        if seconds == 0 {
            println!("Boom!");
            break;
        }
        println!("{seconds} seconds until explosion");
        seconds -= 1;
    }
}
