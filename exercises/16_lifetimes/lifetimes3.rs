// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

/*
    While implementing this solution, I got curious on when would TWO lifetimes be specified on a 
    struct (say if it has two slice references on it). After much thought, and playing with the 
    open source code here (https://github.com/tfpk/lifetimekata/blob/main/exercises/05_lifetimes_on_types/solutions/src/lib.rs)
    I concluded, just like the repo author here (https://github.com/tfpk/lifetimekata/blob/main/exercises/05_lifetimes_on_types/README.md#two-lifetimes)
    that you would need to do so, say if the first string (source of first field's reference(s)) 
    outlives the second string (source of second field's reference(s)) in the calling code. Why? 
    1. Okay for starters, the struct itself has to be dropped/inaccessible after one of its 
    reference strings is dropped, because of lifetimes relationships. 
    2. If the both fields on the struct have the same lifetime, then you can't even move the other
    string's references out of the struct before it destroys because same lifetimes mean it can't
    be borrowed.
    3. One can at least move the first string's references (aka first field of the struct) out of 
    the struct - before it is dropped - if the lifetimes are defined different for the fields.
*/
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
