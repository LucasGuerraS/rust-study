fn main() {
    apply_to_jobs("SRE", 35);
    let even = is_even(8);
    let odd = is_even(7);
    println!("{even} {odd}");
    let full = alphabets("zeeda");
    let no = alphabets("ghf");
    let half = alphabets("abcd");
    dbg!("{:?} {:?} {:?}", full, no, half);
}

fn apply_to_jobs(title: &str, num: i32) {
    println!("I'm applying to {num} {title} jobs");
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn alphabets(text: &str) -> (bool, bool) {
    (
        text.contains("a") || text.contains("A"),
        text.contains("z") || text.contains("Z"),
    )
}
