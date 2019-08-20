// The 'a reads `the lifetime a'.
// https://doc.rust-lang.org/1.9.0/book/lifetimes.html
//We read &mut i32 as `a mutable reference to an i32' and &'a mut i32
//as `a mutable reference to an i32 with the lifetime 'a'.

struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 {
        self.x
    }
}

fn main() {
    println!("rust lifetimes");
    let y = &5; // this is the same as `let _y = 5; let y = &_y;`
    let f = Foo { x: y };

    println!("x is: {}", f.x());
}
