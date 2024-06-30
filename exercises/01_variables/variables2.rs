// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    /*
    Q: I've begun learning the Rust programming language. Why does Rust need the programmer to 
    specify the type of a variable when declaring it, and also initialize it with some value? 
    Feels very different than say a language like Python?

    Ans:
    1. Memory Safety: Rust is designed to prevent common programming errors like null pointer 
    dereferences, data races, and buffer overflows. By requiring the programmer to specify the type 
    of a variable, Rust can ensure that the program is using the correct amount of memory for that 
    type, and that the variable is initialized before it is used. This helps to prevent undefined 
    behavior and crashes.
    2. Performance: Rust is designed to be a high-performance language, and specifying the type of a 
    variable allows the compiler to optimize the code for that specific type. 
    3. Error Prevention: By requiring the programmer to initialize a variable with some value, Rust 
    can prevent errors caused by using uninitialized variables. This helps to catch errors early in 
    the development process, making it easier to write correct code.
     */
    let x: i32 = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
