fn main() {
    examples();
}

fn examples() {
    println!("{}", identity(5));
}

fn identity<T>(value: T) -> T {
    value
}
