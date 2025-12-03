fn main() {
    let cereals = [
        String::from("Foo"),
        String::from("Bar"),
        String::from("Buzz"),
        String::from("Zit"),
        String::from("Iaw"),
    ];

    let first_two = &cereals[..2];
    println!("{first_two:?}");
    let mid_three = &cereals[2..5];
    println!("{mid_three:?}");
    let foo = &cereals[0];
    let f_slice = &foo[0..1];
    println!("{f_slice:?}");
    examples();
}

fn examples() {
    // String slice
    let language = String::from("Rustacian");
    let slice = &language[0..4];
    println!("{slice}");

    // array slice
    let values = [10, 51, 18, 9];
    let my_slice = &values[0..2];
    println!("{my_slice:?}");
}
