fn main() {
    println!("rust lifetimes");
}
// implicit
fn foo(x: &i32) {}

// explicit
fn bar<'a>(x: &'a i32) {}
