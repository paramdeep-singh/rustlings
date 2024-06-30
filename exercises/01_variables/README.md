# Variables

In Rust, variables are immutable by default.
When a variable is immutable, once a value is bound to a name, you can’t change that value.
You can make them mutable by adding `mut` in front of the variable name.

## Further information

- [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

## My Notes!

##### Unveiling the magic of "shadowing"
The OS knows nothing about "shadowing" - it is a purely a construct of the Rust compiler. So a 
Rust programs with shadowing, and a corresponding Rust program with different variable names (for
each subsequent shadowing) will behave exactly the same. Only difference is that Rust explicitly 
makes the "shadowED" variable inaccessible. Otherwise, in terms of lifecycle and stuff, just treat
"shadowED" and "shadowING" variables as two different variables (with different names), with the
former now inaccessible (enforced by the Rust compiler). 
References: https://www.reddit.com/r/rust/comments/17d6o4v/im_confused_about_shadowing/
