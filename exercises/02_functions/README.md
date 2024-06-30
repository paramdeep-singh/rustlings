# Functions

Here, you'll learn how to write functions and how the Rust compiler can help you debug errors even
in more complex code.

## Further information

- [How Functions Work](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

## My notes
- In Rust, there are expressions, and statements. 
    Statements: do stuff, don't return anything.
    Expression: return what they evaluate to.
- Putting a semicolon `;` at the end turns an expression into a statement.
- In Rust, functions and code-blocks (like the if construct), must evaluate to an expression. The
way they do that is either:
    a) evaluate to the last expression in their body
    b) evaluate earlier if there is a `return` statement somewhere
    c) in case there is no `return` encountered, and the final statement isn't an expression ether,
    then the compiler assumes () is returned - which might be in conflict with a function's return
    type, or different branches of an expression like the if-construct.
