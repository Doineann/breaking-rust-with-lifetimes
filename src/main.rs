#![forbid(unsafe_code)] // Just to enforce safe Rust

fn main() {
    let borrow: &Vec<i32>; // Create a reference
    {
        let value: Vec<i32> = vec![69, 420]; // Create a vector of i32s
        borrow = extend_lifetime(&value); // Illegally 'extend' the lifetime of &value and assign it to borrow
        println!("still valid borrow: {borrow:?}"); // Print the borrowed vector
    }
    println!("illegal borrow: {borrow:?}"); // This should NOT be possible or allowed in safe Rust !!
}

fn weird<'a, 'b, T>(_witness: &'b &'a (), any_borrow: &'a T) -> &'b T {
    any_borrow
}

fn extend_lifetime<'a, 'b, T>(borrowed_ref: &'a T) -> &'b T {
    const FOREVER: &&() = &&(); // Effectively: `const FOREVER: &'static &'static () = &&();`

    let weird_function: fn(&'static &'static (), &'a T) -> &'b T = weird; // Let's do ugly things with `weird`

    weird_function(FOREVER, borrowed_ref) // This will (illegally) change the lifetime of borrowed_ref to `'static`
}
