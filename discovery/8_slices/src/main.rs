fn main() {
    // String slice
    let language = String::from("Rustacian");
    let slice = &language[0..4];
    println!("{slice}");

    // array slice
    let values = [10, 51, 18, 9];
    let my_slice = &values[0..2];
    println!("{my_slice:?}");
}
