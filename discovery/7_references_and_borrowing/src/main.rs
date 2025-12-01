fn main() {
    let mut trip = start_trip();
    add_location(&mut trip, "Boston");
    trip.push_str(" and ");
    add_location(&mut trip, "New York");
    trip.push_str(" and ");
    add_location(&mut trip, "Philly");
    show_intinerary(&trip);
    // use a heap value multiple times without transfering ownership
    println!("{trip}");
    references();
    ownership_arrays();
}

fn start_trip() -> String {
    String::from("The plan is...")
}

fn add_location(text: &mut String, location: &str) {
    text.push_str(location);
}

fn show_intinerary(trip: &String) {
    println!("{trip}");
}

fn ownership_arrays() {
    let regs = [true, false, true];
    let first = regs[0];
    println!("{first}");

    /*  Cannot have a reference be a copy of it's value because it's a heap value and
    it would remove ownership of a section of a composite type
    */
    let langs = [String::from("Java"), String::from("Go")];
    let lf = &langs[0];
    println!("{lf}");
}

fn references() {
    let mut current_meal = String::new();
    //^ meal: String -> takes ownership
    //^ mut meal: String -> takes ownership with mutable
    //^ meal: &String -> does not take ownership, copies address
    //^ meal: &mut String -> does not take ownership, but can make changes to the value at the address
    add_flower(&mut current_meal);
    show_my_meal(&current_meal);
}

fn add_flower(meal: &mut String) {
    meal.push_str("Fooo");
}

fn show_my_meal(meal: &String) {
    println!("{meal}");
}
