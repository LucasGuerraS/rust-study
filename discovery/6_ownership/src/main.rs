fn main() {
    println!("---- main function ----\n");

    let meal = String::from("Tuna");
    println!("I have a grat dish of {meal}");
    eat_meal(meal);
    ownership_basics();
    ownership_with_functions();
}

fn eat_meal(mut meal: String) {
    println!("Eating {meal}");
    meal.clear();
    println!("Now that it's eaten there is {meal} left");
}

fn ownership_basics() {
    println!("\n---- Examples of ownership and memory operators ----- \n");

    //& basis of the String type
    let _foo = String::new();
    let bar = String::from("Buzz");
    println!("{bar}");

    //& Mutability with &str
    let mut abc = "dadwad";
    println!("{abc}");
    abc = "fawadawwadawd";
    println!("{abc}");

    //& Moves and ownership demonstrated
    let person = String::from("Lucas");
    println!("{person}");
    let mut genius = person; // genius becomes the owner of the heap value to avoid duplication
    println!("{}", genius);
    genius.push_str(" Guerra");
    println!("{genius}");
    //^ drop(genius); -> manually drop a variable, this would be done automatically by the end of the main fn

    //& References and borrowing
    let borrow_genius = &genius;
    let borrow_genius_again = &genius; // references implement the copy trait
    println!("Reference is: {}", *borrow_genius); // doesn't need * because references implement display trait
    println!("Reference copy is: {}", *borrow_genius_again);
}

fn ownership_with_functions() {
    println!("---- Examples of ownership in functions ----- \n");

    // Ownership with integers and functions
    let apples = 64; // ownership is maintained even after the function execution
    print_my_value(apples);

    // Ownership with heap values
    let my_text = String::from("Guatemala");
    let text_ref = &my_text;
    print_my_string_ref(text_ref); // avoiding removing the variable of scope with references
    print_my_string(my_text);
    //^ println!("{}", my_text); -> compile error because the parameter took ownership of the value

    println!("\n---- End of examples of ownership in functions ----- \n");
}

fn print_my_value(value: i32) {
    println!("Value is {value}");
}

fn print_my_string(text: String) {
    println!("Your text is {text}");
}

fn print_my_string_ref(reference: &String) {
    println!("Your reference is {reference}")
}
