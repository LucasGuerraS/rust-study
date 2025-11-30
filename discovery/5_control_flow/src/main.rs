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
