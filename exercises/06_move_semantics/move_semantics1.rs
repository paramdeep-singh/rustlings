// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    // stack variable 'vec0' now points to a vector allocated in the heap
    let vec0 = vec![22, 44, 66];

    // vec0 lost ownership here
    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec; // shadowing to mutable

    vec.push(88);

    // here the ownership of the heap allocated vector is passed to the calling function, and so
    // the vector is not dropped in the heap since the owner is not here
    vec 
}
