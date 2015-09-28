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
## Usage

* Clone the repository
* Build the library

```shell
$ rustc --crate-type=lib src/merge_sort.rs
```

* Add the crate to your project

```
extern crate merge_sort;
```

* Sort your list!

```
fn main() {
  let mut list: Vec<i32> = vec![234234, 9, 4, 5, 99, 1,-3, 0, -1];
  
  // Print the original vector to STDOUT
  println!("Original list: {:?}", example_list);

  // Merge the list
  merge_sort::sort(&mut example_list);

  // Print the sorted vector to STDOUT
  println!("Sorted list: {:?}", example_list);
}
``` 

Example

```shell
$ rustc --crate-type=lib src/merge_sort.rs; rustc -L . src/example.rs; ./example
Original list: [234234, 9, 4, 5, 99, 1, -3, 0, -1]
Sorted list: [-3, -1, 0, 1, 4, 5, 9, 99, 234234]
```
