// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

/*
    Constants in Rust must have their type specified because they are evaluated at compile-time, 
    while variables defined with let can have their type inferred because they are evaluated at 
    runtime. Thus constants are compile-time creatures, and the compiler needs all the support 
    possible to understand its value, type, and then effectively paste it wherever it is referenced.
 */
const NUMBER: i32 = 3;
fn main() {
    /*
        Wanted to experiment if Rust "shadowing" works for constants as well, and to my surprise, it 
        does! After some reading, it does make sense: from a compiler's perspective it just needs
        to paste the constant value wherever it is reference in the code, so there shouldn't be any
        problem if the programmer at some point wants to "update" what is meant by that constant
        name. The compiler will just paste that "new" value denoted by the constant name going 
        forward. Also, reference the notes under "Unveiling the magic of "shadowing" under this 
        folder's README
     */
    // const NUMBER: i32 = 4;
    println!("Number {}", NUMBER);
}
