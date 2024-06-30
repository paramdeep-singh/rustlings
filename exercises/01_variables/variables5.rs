// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // This is called "shadowing" in rust! To note: after this transformation the variable is again 
    // immutable! 
    let number = 3; // don't rename this variable
    // number = 2; ... so this won't work!
    println!("Number plus two is : {}", number + 2);
}
