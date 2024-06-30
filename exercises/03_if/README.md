# If

`if`, the most basic (but still surprisingly versatile!) type of control flow, is what you'll learn here.

## Further information

- [Control Flow - if expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)

## Regarding return in an `if` statement:
Consider the following statement:
```
let a = if (amount < 3) { return 3 * amount };
```
Initially, I found it very difficult to reason through it. I thought the compiler would complain 
given the types of `3 * amount` and `()` would be incompatible. However, the compiler was just fine
with this code! why? I think this is because of the nature of `return` statement (as described here
: https://doc.rust-lang.org/reference/expressions/return-expr.html), once `return` is met, Rust will    
simply exit the function, and so it simply isn't necessary to even evaluate the expression of the 
enclosing block! Coming back to the code above, as far as the Rust compiler is concerned, the
variable `a` can only ever be of type '()'.
