#[link(name = "calculate", kind = "static")]
extern "C" {
    fn calculate_add_three(n: i32) -> i32;
}

fn main() {
    unsafe {
        println!("{}", calculate_add_three(7));
    }
}
