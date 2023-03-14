// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.


fn main() {
    let answer: u32 = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: u32) -> u32 { num * num }
