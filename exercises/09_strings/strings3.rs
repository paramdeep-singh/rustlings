// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

/*
    I asked ChatGPT what is going on here memory wise, and they gave a beautiful answer!
    
    Answer:
    Certainly! Let's break down what happens in memory when you call the `trim_me` function in Rust, focusing on stack and heap allocations:

### Memory Allocation: Stack and Heap

1. **Stack Allocation**:
   - When you call the function `trim_me` with a string slice (`&str`), the reference to the string slice is passed to the function. This reference (which includes a pointer to the start of the string data and the length of the slice) is stored on the stack.
   - The function's local variables and the return address are also stored on the stack.

2. **Heap Allocation**:
   - String slices (`&str`) themselves do not own their data; they are simply references to some portion of a string. The actual string data that the slice points to is typically stored on the heap, especially if it's a `String` object from which the slice was derived.
   - When you call `input.trim()`, this method returns a new string slice that still references the original string data on the heap. No new string data is created here; the new slice just potentially points to a different starting point and might have a different length if there were leading or trailing whitespaces.
   - The call to `.to_string()` on the trimmed slice results in the creation of a new `String` object. This involves:
     - Allocating memory on the heap to hold the new string data.
     - Copying the data from the trimmed slice into this new heap allocation.
     - The `String` object itself (storing the pointer to the heap data, the length of the string, and its capacity) is returned by the function and is typically stored on the stack.

### Summary

- **Stack**: Stores the function's parameters, local variables, and the `String` object's metadata (pointer, length, capacity).
- **Heap**: Stores the actual character data of the string. Initially, the original string data is on the heap, and a new heap allocation is made for the `String` returned by `.to_string()`.

This separation allows Rust to efficiently manage memory and ensures safety through its ownership and borrowing rules, preventing data races and memory corruption. The use of slices and the distinction between `String` (owner of data) and `&str` (borrowed reference) are central to these mechanisms.
*/
fn trim_me(input: &str) -> String {
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this!
    let mut composed_input = input.to_string();
    composed_input.push_str(" world!");
    composed_input
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
