# Merge Sort in Rust

Rust seems like a cool up and coming systems level language. Some things that stand out differentiating Rust from other similar languages:

* Concept of 'ownership' vs 'references'/'borrowing'
* Just about everything is an expression (opposed to a statement)
* Powerful macros
* All bindings by default are immutable
* Rust is statically typed, but has an inference system to balance out the power of static typing with the verbosity of annotating types.
* Object memory allocation is freed as soon as the binding goes out of scope
* Cargo for Rust project/dependency management (still a WIP)
* Much more...

While Rust seems to be like a fairly strong and easy language to grasp, it's still young and much of the API is still a WIP and should not be used. There are also lingering issues with massive memory leaks.

Read more about [Rust](https://doc.rust-lang.org)

So lets write your typical merge sort implementation in Rust!

## Usage

* Clone the repository
* Test the code

```bash
$ cargo test
```

* Check out the example at [examples/sort.rs](examples/sort.rs):

```rust
extern crate rust_merge_sort;

fn main() {
    let mut list: Vec<i32> = vec![234234, 9, 4, 5, 99, 1,-3, 0, -1];
    
    // Print the original vector to STDOUT
    println!("Original list: {:?}", example_list);

    // Merge the list
    rust_merge_sort::sort(&mut example_list);

    // Print the sorted vector to STDOUT
    println!("Sorted list: {:?}", example_list);
}
``` 

Which outputs:

```bash
$ cargo run --example sort
Original list: [234234, 9, 4, 5, 99, 1, -3, 0, -1]
Sorted list: [-3, -1, 0, 1, 4, 5, 9, 99, 234234]
```

* Add it as dependency to your `Cargo.toml` and start using it in your project:

```toml
[dependencies]
rust_merge_sort = { version = "*", path = "path/to/rust-merge-sort" }
```
